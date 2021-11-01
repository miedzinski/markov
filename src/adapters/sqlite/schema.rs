use cached::proc_macro::cached;
use rusqlite::Connection;
use sql_builder::{name, SqlBuilder, SqlName};

fn word_fk(nth: usize) -> String {
    format!("word_{}_id", nth)
}

pub fn setup<const N: usize>(connection: &Connection) -> anyhow::Result<()> {
    let word_fk_defs = (0..N)
        .map(|i| format!("{} INTEGER NOT NULL REFERENCES word (id)", word_fk(i)))
        .collect::<Vec<_>>()
        .join(", ");
    let word_fks = (0..N).map(word_fk).collect::<Vec<_>>().join(", ");
    let sql = format!(
        "\
BEGIN;
CREATE TABLE word (
    id INTEGER PRIMARY KEY,
    value TEXT NOT NULL UNIQUE
);

CREATE TABLE transition_from (
    id INTEGER PRIMARY KEY,
    {},
    UNIQUE ({})
);

CREATE TABLE transition (
    transition_from_id INTEGER NOT NULL REFERENCES transition_from (id),
    to_id INTEGER NOT NULL REFERENCES word (id),
    weight INTEGER NOT NULL,
    PRIMARY KEY (transition_from_id, to_id),
    CHECK (weight > 0)
);
COMMIT;",
        word_fk_defs, word_fks
    );
    connection.execute_batch(&sql)?;
    Ok(())
}

#[cached]
pub fn get_word() -> String {
    SqlBuilder::select_from("word")
        .field("id")
        .and_where_eq("value", "?")
        .sql()
        .unwrap()
}

#[cached]
pub fn insert_word() -> String {
    SqlBuilder::insert_into("word")
        .field("value")
        .values(&["?"])
        .sql()
        .unwrap()
}

#[cached]
pub fn get_transition_from(n: usize) -> String {
    (0..n)
        .fold(
            SqlBuilder::select_from("transition_from").field("id"),
            |builder, i| builder.and_where_eq(word_fk(i), "?"),
        )
        .sql()
        .unwrap()
}

#[cached]
pub fn insert_transition_from(n: usize) -> String {
    (0..n)
        .fold(
            &mut SqlBuilder::insert_into("transition_from"),
            |builder, i| builder.field(word_fk(i)),
        )
        .values(&vec!["?"; n])
        .sql()
        .unwrap()
}

#[cached]
pub fn increment_weight() -> String {
    let mut sql = SqlBuilder::insert_into("transition")
        .fields(&["transition_from_id", "to_id", "weight"])
        .values(&["?", "?", "1"])
        .sql()
        .unwrap();
    if sql.ends_with(';') {
        sql.pop();
    }
    sql.push_str(" ON CONFLICT (transition_from_id, to_id) DO UPDATE SET weight = weight + 1;");
    sql
}

#[cached]
pub fn get_weights(n: usize) -> String {
    (0..n)
        .fold(
            SqlBuilder::select_from(name!("transition_from"; "tf"))
                .fields(&["w.value", "t.weight"]),
            |builder, i| {
                let alias = format!("w{}", i);
                builder
                    .join(name!("word"; &alias))
                    .on_eq(format!("{}.id", &alias), format!("tf.{}", word_fk(i)))
                    .and_where_eq(format!("{}.value", &alias), "?")
            },
        )
        .join(name!("transition"; "t"))
        .on_eq("t.transition_from_id", "tf.id")
        .join(name!("word"; "w"))
        .on_eq("w.id", "t.to_id")
        .sql()
        .unwrap()
}

#[cached]
pub fn get_random(n: usize, filter_starts_with: bool) -> String {
    let max_rowid = SqlBuilder::select_from("transition_from")
        .field("max(rowid)")
        .subquery_as("max_rowid")
        .unwrap();
    let mut builder = SqlBuilder::select_from(name!("transition_from"; "tf"));
    let mut builder = (0..n)
        .fold(
            &mut builder,
            |builder, i| {
                let alias = format!("w{}", i);
                builder
                    .join(name!("word"; &alias))
                    .on_eq(format!("{}.id", &alias), format!("tf.{}", word_fk(i)))
                    .field(format!("{}.value", &alias))
            },
        )
        .field(max_rowid)
        .and_where("tf.rowid >= abs(random()) % max_rowid");
    if filter_starts_with {
        builder = builder.and_where_eq(word_fk(0), "?");
    }
    builder.sql().unwrap()
}
