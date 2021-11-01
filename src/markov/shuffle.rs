pub trait Shuffle<T> {
    fn shuffle(&self, slice: &mut [T]);
}
