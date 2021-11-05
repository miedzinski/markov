use anyhow::Result;
use clap::{App, Arg};
use rusqlite::Connection;

use crate::adapters::discord::bot::DiscordBot;
use crate::adapters::memory::MemoryRepository;
use crate::adapters::rand::choose::RandChoose;
use crate::adapters::rand::shuffle::RandShuffle;
use crate::adapters::sqlite::repository::SqliteRepository;
use crate::adapters::sqlite::schema::setup;
use crate::markov::bot::Bot;
use crate::markov::chain::Chain;

mod adapters;
mod markov;

const ORDER: usize = 2;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("markov")
        .arg(
            Arg::with_name("sqlite-path")
                .long("sqlite-path")
                .takes_value(true)
                .help("Path to SQLite database"),
        )
        .arg(
            Arg::with_name("setup-db")
                .long("setup-db")
                .requires("sqlite"),
        )
        .arg(
            Arg::with_name("token")
                .long("token")
                .help("Discord token")
                .takes_value(true)
                .required_unless("setup-db"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .takes_value(true)
                .help("Verbosity level (number in range 0..1)")
                .default_value("0.05")
                .validator(|verbosity| match verbosity.parse::<f64>() {
                    Err(_) => Err("must be a number".to_string()),
                    Ok(v) if !(0.0..1.0).contains(&v) => Err("must be in range 0..1".to_string()),
                    _ => Ok(()),
                }),
        )
        .get_matches();

    let connection = matches.value_of("sqlite").map(Connection::open);

    if matches.is_present("setup-db") {
        return setup::<ORDER>(&connection.unwrap()?);
    }

    let token = matches.value_of("token").unwrap();
    let verbosity = matches
        .value_of("verbosity")
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let chooser = RandChoose::new();
    let shuffler = RandShuffle::new();

    match connection {
        Some(connection) => {
            let repository = SqliteRepository::new(connection?);
            let chain = Chain::new(repository, chooser);
            let bot: Bot<_, _, _, 2> = Bot::new(chain, shuffler);
            let discord = DiscordBot::new();
            discord.run(bot, token, verbosity).await
        }
        None => {
            let repository = MemoryRepository::new();
            let chain = Chain::new(repository, chooser);
            let bot: Bot<_, _, _, 2> = Bot::new(chain, shuffler);
            let discord = DiscordBot::new();
            discord.run(bot, token, verbosity).await
        }
    }
}
