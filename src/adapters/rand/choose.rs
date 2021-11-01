use rand::{thread_rng, Rng};

use crate::markov::choose::Choose;

pub struct RandChoose;

impl RandChoose {
    pub fn new() -> RandChoose {
        RandChoose
    }
}

impl<T> Choose<T> for RandChoose {
    fn generate_random(&self, upper_bound: u32) -> u32 {
        let mut rng = thread_rng();
        rng.gen::<u32>() % upper_bound
    }
}
