use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::markov::shuffle::Shuffle;

pub struct RandShuffle;

impl RandShuffle {
    pub fn new() -> RandShuffle {
        RandShuffle
    }
}

impl<T> Shuffle<T> for RandShuffle {
    fn shuffle(&self, slice: &mut [T]) {
        let mut rng = thread_rng();
        slice.shuffle(&mut rng);
    }
}
