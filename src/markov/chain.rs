use super::choose::Choose;
use super::links::LinkIterator;
use super::repository::Repository;

pub struct Chain<R, C> {
    repository: R,
    chooser: C,
}

impl<R, C> Chain<R, C> {
    pub fn new(repository: R, chooser: C) -> Chain<R, C> {
        Chain {
            repository,
            chooser,
        }
    }

    pub fn feed<T, I, const N: usize>(&mut self, iter: I)
    where
        T: Clone,
        R: Repository<T, N>,
        I: IntoIterator<Item = T>,
    {
        for link in iter.into_iter().links() {
            self.repository.increment_weight(link);
        }
    }

    pub fn iter_from<T, const N: usize>(&self, start: [T; N]) -> ChainIterator<T, C, N>
    where
        R: Repository<T, N>,
        C: Choose<T> + Clone,
    {
        ChainIterator {
            repository: &self.repository,
            chooser: self.chooser.clone(),
            previous: start,
        }
    }

    pub fn iter_random<T, const N: usize>(&self) -> Option<ChainIterator<T, C, N>>
    where
        R: Repository<T, N>,
        C: Choose<T> + Clone,
    {
        let start = self.repository.random();
        start.map(|start| self.iter_from(start))
    }
}

pub struct ChainIterator<'a, T, S, const N: usize> {
    repository: &'a dyn Repository<T, N>,
    chooser: S,
    previous: [T; N],
}

impl<'a, T, S, const N: usize> Iterator for ChainIterator<'a, T, S, N>
where
    T: Clone,
    S: Choose<T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let weights = self.repository.get(&self.previous);
        if weights.is_empty() {
            return None;
        }
        let state = self
            .chooser
            .choose(weights.iter().map(|(state, &weight)| (state, weight)));
        self.previous.rotate_left(1);
        self.previous[N - 1] = state.clone();
        Some(state)
    }
}
