use std::cell::RefCell;

use rand::{Rng, RngCore};

use crate::markov::choose::Choose;

#[derive(Clone)]
pub struct RandChoose<R> {
    rng: RefCell<R>,
}

impl<R> RandChoose<R> {
    pub fn new(rng: R) -> RandChoose<R> {
        RandChoose {
            rng: RefCell::new(rng),
        }
    }
}

impl<T, G> Choose<T> for RandChoose<G>
where
    G: RngCore,
{
    fn generate_random(&self, upper_bound: u32) -> u32 {
        let mut rng = self.rng.borrow_mut();
        (*rng).gen::<u32>() % upper_bound
    }
}
