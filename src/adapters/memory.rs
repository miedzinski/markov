use std::collections::HashMap;
use std::hash::Hash;

use rand::seq::IteratorRandom;
use rand::thread_rng;

use crate::markov::repository::Repository;
use crate::markov::types::{Link, WeightMap};

pub struct MemoryRepository<T, const N: usize> {
    chain: HashMap<[T; N], WeightMap<T>>,
    empty: WeightMap<T>,
}

impl<T, const N: usize> MemoryRepository<T, N> {
    pub fn new() -> MemoryRepository<T, N> {
        MemoryRepository {
            chain: HashMap::new(),
            empty: WeightMap::new(),
        }
    }
}

impl<T, const N: usize> Repository<T, N> for MemoryRepository<T, N>
where
    T: Clone + Eq + Hash,
{
    fn get(&self, from: &[T; N]) -> &WeightMap<T> {
        self.chain.get(from).unwrap_or(&self.empty)
    }

    fn random(&self) -> Option<[T; N]> {
        let mut rng = thread_rng();
        self.chain.keys().choose(&mut rng).cloned()
    }

    fn increment_weight(&mut self, link: Link<T, N>) {
        let Link { from, to } = link;
        let weights = self.chain.entry(from).or_insert_with(WeightMap::new);
        weights.entry(to).and_modify(|x| *x += 1).or_insert(1);
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

        assert_eq!(repository.get(&[1, 2, 3]), &WeightMap::new());
    }

    #[test]
    fn get_returns_requested_map() {
        let mut repository: MemoryRepository<i32, 3> = MemoryRepository::new();
        let map = WeightMap::new();
        repository.chain.insert([1, 2, 3], map.clone());

        assert_eq!(repository.get(&[1, 2, 3]), &map);
    }

    #[test]
    fn increment_weight_sets_weight_to_1_if_missing() {
        let mut repository: MemoryRepository<i32, 3> = MemoryRepository::new();
        let link = Link::new([1, 2, 3], 4);
        repository.increment_weight(link);

        assert_eq!(repository.chain[&[1, 2, 3]][&4], 1);
    }

    #[test]
    fn increments_weight_by_1() {
        let mut repository: MemoryRepository<i32, 3> = MemoryRepository::new();
        let mut map = WeightMap::new();
        map.insert(4, 1);
        repository.chain.insert([1, 2, 3], map);
        let link = Link::new([1, 2, 3], 4);
        repository.increment_weight(link);

        assert_eq!(repository.chain[&[1, 2, 3]][&4], 2);
    }
}
