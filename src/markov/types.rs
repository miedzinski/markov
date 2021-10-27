use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Link<T, const N: usize> {
    pub from: [T; N],
    pub to: T,
}

impl<T, const N: usize> Link<T, N> {
    pub fn new(from: [T; N], to: T) -> Link<T, N> {
        Link { from, to }
    }
}

pub type WeightMap<T> = HashMap<T, u32>;
