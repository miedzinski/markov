use std::iter;

use anyhow::{Context, Result};

use super::chain::Chain;
use super::choose::Choose;
use super::repository::Repository;
use super::shuffle::Shuffle;

static END: &str = "\0";

pub struct Bot<R, C, S, const N: usize>
where
    R: Repository<String, N>,
    C: Choose<String>,
{
    chain: Chain<String, R, C, N>,
    shuffler: S,
}

impl<R, C, S, const N: usize> Bot<R, C, S, N>
where
    R: Repository<String, N>,
    C: Choose<String>,
{
    pub fn new(chain: Chain<String, R, C, N>, shuffler: S) -> Bot<R, C, S, N> {
        Bot { chain, shuffler }
    }

    pub fn learn(&mut self, message: &str) -> Result<()> {
        let words = message
            .split_whitespace()
            .chain(iter::once(END))
            .map(str::to_string);
        self.chain.feed(words)?;
        Ok(())
    }

    fn build_sentence(&self, start: [String; N]) -> Result<String> {
        start
            .clone()
            .into_iter()
            .map(Ok)
            .chain(self.chain.iter_from(start))
            .take_while(|word| !matches!(word, Ok(word) if word == END))
            .collect::<Result<Vec<_>>>()
            .map(|words| words.join(" "))
    }

    pub fn say(&self) -> Result<String> {
        self.chain
            .random()?
            .context("Failed to build random sentence.")
            .and_then(|start| self.build_sentence(start))
    }

    pub fn reply(&self, message: &str) -> Result<String>
    where
        S: Shuffle<String>,
    {
        let mut words: Vec<_> = message.split_whitespace().map(str::to_string).collect();
        self.shuffler.shuffle(&mut words);

        for word in words {
            let start = self.chain.random_starting_with(&word.to_string())?;
            if let Some(start) = start {
                return self.build_sentence(start);
            }
        }

        self.say()
    }
}
