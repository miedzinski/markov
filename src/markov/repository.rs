use crate::markov::types::{Link, WeightMap};

pub trait Repository<T, const N: usize> {
    fn get(&self, from: &[T; N]) -> &WeightMap<T>;
    fn random(&self) -> Option<[T; N]>;
    fn increment_weight(&mut self, link: Link<T, N>);
}
