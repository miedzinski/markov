use crate::markov::types::WeightMap;

pub trait Choose<T> {
    fn choose(&self, map: WeightMap<T>) -> T {
        let sum = map.values().sum();
        let random = self.generate_random(sum);
        let mut i = 0;
        for (state, weight) in map {
            i += weight;
            if i > random {
                return state;
            }
        }
        unreachable!()
    }

    fn generate_random(&self, upper_bound: u32) -> u32;
}
