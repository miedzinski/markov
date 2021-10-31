use std::hash::Hash;

use anyhow::Result;
use arrayvec::ArrayVec;
use rusqlite::types::FromSql;
use rusqlite::{params_from_iter, Connection, Error, ToSql, Transaction};

use super::schema;
use crate::markov::repository::Repository;
use crate::markov::types::{Link, WeightMap};

pub struct SqliteRepository {
    connection: Connection,
}

impl SqliteRepository {
    pub fn new(connection: Connection) -> SqliteRepository {
        SqliteRepository { connection }
    }
}

impl SqliteRepository {
    fn get_or_create_word<T>(transaction: &Transaction, value: T) -> Result<i64>
    where
        T: ToSql,
    {
        let sql = schema::get_word();
        let result = transaction
            .prepare_cached(&sql)?
            .query_row([&value], |row| row.get(0));
        match result {
            Err(Error::QueryReturnedNoRows) => {
                let sql = schema::insert_word();
                transaction.prepare_cached(&sql)?.execute([value])?;
                let rowid = transaction.last_insert_rowid();
                Ok(rowid)
            }
            result => result.map_err(Into::into),
        }
    }

    fn get_or_create_transition_from<const N: usize>(
        transaction: &Transaction,
        from_ids: &ArrayVec<i64, N>,
    ) -> Result<i64> {
        let sql = schema::get_transition_from(N);
        let params = params_from_iter(from_ids);
        let result = transaction
            .prepare_cached(&sql)?
            .query_row(params.clone(), |row| row.get(0));
        match result {
            Err(Error::QueryReturnedNoRows) => {
                let sql = schema::insert_transition_from(N);
                transaction.prepare_cached(&sql)?.execute(params)?;
                let rowid = transaction.last_insert_rowid();
                Ok(rowid)
            }
            result => result.map_err(Into::into),
        }
    }

    fn increment_weight(
        transaction: &Transaction,
        transition_from_id: i64,
        to_id: i64,
    ) -> Result<()> {
        let sql = schema::increment_weight();
        let params = [transition_from_id, to_id];
        transaction.prepare_cached(&sql)?.execute(params)?;
        Ok(())
    }
}

impl<T, const N: usize> Repository<T, N> for SqliteRepository
where
    T: FromSql + ToSql + Hash + Eq,
{
    fn get(&self, from: &[T; N]) -> Result<WeightMap<T>> {
        let sql = schema::get_weights(N);
        let params = params_from_iter(from);
        let map = self
            .connection
            .prepare_cached(&sql)?
            .query_and_then(params, |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<_>>()?;
        Ok(map)
    }

    fn random(&self) -> Result<[T; N]> {
        let sql = schema::get_random(N);
        self.connection
            .prepare_cached(&sql)?
            .query_and_then([], |row| {
                let mut words: ArrayVec<_, N> = ArrayVec::new();
                for idx in 0..N {
                    words.push(row.get(idx)?);
                }
                let words = unsafe {
                    // This is safe, because we've just pushed N items.
                    // Using unchecked variant allows us to omit T: Debug.
                    words.into_inner_unchecked()
                };
                Ok(words)
            })?
            .next()
            .unwrap()
    }

    fn increment_weight(&mut self, link: Link<T, N>) -> Result<()> {
        let transaction = self.connection.transaction()?;
        let from_ids = (0..N)
            .map(|i| Self::get_or_create_word(&transaction, &link.from[i]))
            .collect::<Result<ArrayVec<_, N>>>()?;
        let transition_from_id = Self::get_or_create_transition_from(&transaction, &from_ids)?;
        let to_id = Self::get_or_create_word(&transaction, link.to)?;
        Self::increment_weight(&transaction, transition_from_id, to_id)?;
        transaction.commit()?;
        Ok(())
    }
}
