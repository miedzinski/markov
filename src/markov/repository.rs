use anyhow::Result;

use super::types::{Link, WeightMap};

pub trait Repository<T, const N: usize> {
    fn get(&self, from: &[T; N]) -> Result<WeightMap<T>>;
    fn random(&self) -> Result<Option<[T; N]>>;
    fn random_starting_with(&self, state: &T) -> Result<Option<[T; N]>>;
    fn increment_weight(&mut self, link: Link<T, N>) -> Result<()>;
}
