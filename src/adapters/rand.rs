use rand::{Rng, RngCore};

use crate::markov::choose::Choose;
use std::cell::RefCell;

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

#[cfg(test)]
mod tests {
    use rand::rngs::mock::StepRng;

    use super::RandChoose;
    use crate::markov::choose::Choose;

    fn step_sampler() -> RandChoose<StepRng> {
        RandChoose::new(StepRng::new(0, 2))
    }

    #[test]
    fn samples() {
        let sampler = step_sampler();
        let weights = [("foo", 3), ("bar", 2), ("baz", 1)];
        let iter = weights.iter().map(|(a, b)| (a, *b));

        assert_eq!(*sampler.choose(iter.clone()), "foo");
        assert_eq!(*sampler.choose(iter.clone()), "foo");
        assert_eq!(*sampler.choose(iter.clone()), "bar");
        assert_eq!(*sampler.choose(iter), "foo");
    }
}
