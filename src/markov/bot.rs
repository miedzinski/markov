use std::iter;

use anyhow::{Context, Result};

use super::chain::Chain;
use super::shuffle::Shuffle;

static END: &str = "\0";

pub struct Bot<'a, S, const N: usize> {
    chain: Chain<'a, String, N>,
    shuffler: S,
}

impl<'a, S, const N: usize> Bot<'a, S, N> {
    pub fn new(chain: Chain<'a, String, N>, shuffler: S) -> Bot<'a, S, N> {
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

    pub fn reply<'c>(&self, message: &'c str) -> Result<String>
    where
        S: Shuffle<&'c str>,
    {
        let mut words: Vec<_> = message.split_whitespace().collect();
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
