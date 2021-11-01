use anyhow::Result;

use super::choose::Choose;
use super::links::LinkIterator;
use super::repository::Repository;

pub struct Chain<'a, T, const N: usize> {
    repository: &'a mut dyn Repository<T, N>,
    chooser: &'a dyn Choose<T>,
}

impl<T, const N: usize> Chain<'_, T, N> {
    pub fn new<'a>(
        repository: &'a mut dyn Repository<T, N>,
        chooser: &'a dyn Choose<T>,
    ) -> Chain<'a, T, N> {
        Chain {
            repository,
            chooser,
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
            repository: self.repository,
            chooser: self.chooser,
            previous: start,
        }
    }

    pub fn iter_random(&self) -> Result<ChainIterator<T, N>> {
        self.repository.random().map(|start| self.iter_from(start))
    }

    pub fn iter_from_state(&self, state: &T) -> Result<ChainIterator<T, N>> {
        self.repository
            .random_starting_with(state)
            .map(|start| self.iter_from(start))
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
