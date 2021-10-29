use anyhow::Result;

use crate::markov::types::{Link, WeightMap};

pub trait Repository<T, const N: usize> {
    fn get(&self, from: &[T; N]) -> Result<WeightMap<T>>;
    fn random(&self) -> Result<[T; N]>;
    fn increment_weight(&mut self, link: Link<T, N>) -> Result<()>;
}
