use std::marker::PhantomData;

use anyhow::Result;

use super::choose::Choose;
use super::links::LinkIterator;
use super::repository::Repository;

pub struct Chain<T, R, C, const N: usize>
where
    R: Repository<T, N>,
    C: Choose<T>,
{
    repository: R,
    chooser: C,
    phantom: PhantomData<[T; N]>,
}

impl<T, R, C, const N: usize> Chain<T, R, C, N>
where
    R: Repository<T, N>,
    C: Choose<T>,
{
    pub fn new(repository: R, chooser: C) -> Chain<T, R, C, N> {
        Chain {
            repository,
            chooser,
            phantom: PhantomData::default(),
        }
    }

    pub fn feed<I>(&mut self, iter: I) -> Result<()>
    where
        T: Clone,
        I: IntoIterator<Item = T>,
    {
        for link in iter.into_iter().links() {
            self.repository.increment_weight(link)?;
        }
        Ok(())
    }

    pub fn iter_from(&self, start: [T; N]) -> ChainIterator<T, N> {
        ChainIterator {
            repository: &self.repository,
            chooser: &self.chooser,
            previous: start,
        }
    }

    pub fn random(&self) -> Result<Option<[T; N]>> {
        self.repository.random()
    }

    pub fn random_starting_with(&self, state: &T) -> Result<Option<[T; N]>> {
        self.repository.random_starting_with(state)
    }
}

pub struct ChainIterator<'a, T, const N: usize> {
    repository: &'a dyn Repository<T, N>,
    chooser: &'a dyn Choose<T>,
    previous: [T; N],
}

impl<T, const N: usize> Iterator for ChainIterator<'_, T, N>
where
    T: Clone,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let weights = self.repository.get(&self.previous);
        match weights {
            Ok(weights) if !weights.is_empty() => {
                let state = self.chooser.choose(weights);
                self.previous.rotate_left(1);
                self.previous[N - 1] = state.clone();
                Some(Ok(state))
            }
            Err(e) => Some(Err(e)),
            _ => None,
        }
    }
}
