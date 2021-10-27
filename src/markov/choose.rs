pub trait Choose<T> {
    fn choose<'a>(&self, states: (impl Iterator<Item = (&'a T, u32)> + Clone)) -> &'a T {
        let sum = states.clone().map(|x| x.1).sum();
        let random = self.generate_random(sum);
        let mut i = 0;
        for (state, weight) in states {
            i += weight;
            if i > random {
                return state;
            }
        }
        unreachable!()
    }

    fn generate_random(&self, upper_bound: u32) -> u32;
}
