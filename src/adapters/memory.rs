use std::collections::HashMap;
use std::hash::Hash;

use anyhow::{Context, Result};
use rand::seq::IteratorRandom;
use rand::thread_rng;

use crate::markov::repository::Repository;
use crate::markov::types::{Link, WeightMap};

pub struct MemoryRepository<T, const N: usize> {
    chain: HashMap<[T; N], WeightMap<T>>,
}

impl<T, const N: usize> MemoryRepository<T, N> {
    pub fn new() -> MemoryRepository<T, N> {
        MemoryRepository {
            chain: HashMap::new(),
        }
    }
}

impl<T, const N: usize> Repository<T, N> for MemoryRepository<T, N>
where
    T: Clone + Eq + Hash,
{
    fn get(&self, from: &[T; N]) -> Result<WeightMap<T>> {
        Ok(self.chain.get(from).cloned().unwrap_or_else(WeightMap::new))
    }

    fn random(&self) -> Result<[T; N]> {
        let mut rng = thread_rng();
        self.chain
            .keys()
            .choose(&mut rng)
            .cloned()
            .context("Failed to choose random states.")
    }

    fn increment_weight(&mut self, link: Link<T, N>) -> Result<()> {
        let Link { from, to } = link;
        let weights = self.chain.entry(from).or_insert_with(WeightMap::new);
        weights.entry(to).and_modify(|x| *x += 1).or_insert(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MemoryRepository;
    use crate::markov::repository::Repository;
    use crate::markov::types::{Link, WeightMap};

    #[test]
    fn get_returns_empty_if_missing() {
        let repository: MemoryRepository<i32, 3> = MemoryRepository::new();

        assert_eq!(repository.get(&[1, 2, 3]).unwrap(), WeightMap::new());
    }

    #[test]
    fn get_returns_requested_map() {
        let mut repository: MemoryRepository<i32, 3> = MemoryRepository::new();
        let map = WeightMap::new();
        repository.chain.insert([1, 2, 3], map.clone());

        assert_eq!(repository.get(&[1, 2, 3]).unwrap(), map);
    }

    #[test]
    fn increment_weight_sets_weight_to_1_if_missing() {
        let mut repository: MemoryRepository<i32, 3> = MemoryRepository::new();
        let link = Link::new([1, 2, 3], 4);
        repository.increment_weight(link).unwrap();

        assert_eq!(repository.chain[&[1, 2, 3]][&4], 1);
    }

    #[test]
    fn increments_weight_by_1() {
        let mut repository: MemoryRepository<i32, 3> = MemoryRepository::new();
        let mut map = WeightMap::new();
        map.insert(4, 1);
        repository.chain.insert([1, 2, 3], map);
        let link = Link::new([1, 2, 3], 4);
        repository.increment_weight(link).unwrap();

        assert_eq!(repository.chain[&[1, 2, 3]][&4], 2);
    }
}
