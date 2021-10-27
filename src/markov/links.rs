use smallvec::SmallVec;

use super::types::Link;

pub trait LinkIterator {
    fn links<T, const N: usize>(self) -> Links<T, Self, N>
    where
        Self: Sized + Iterator<Item = T>,
    {
        Links::new(self)
    }
}

impl<T: Iterator> LinkIterator for T {}

pub struct Links<T, I, const N: usize> {
    iter: I,
    from: SmallVec<[T; N]>,
}

impl<'a, T, I, const N: usize> Links<T, I, N>
where
    I: Iterator<Item = T>,
{
    fn new<II>(iter: II) -> Links<T, I, N>
    where
        II: IntoIterator<Item = T, IntoIter = I>,
    {
        let mut from = SmallVec::new();
        let mut iter = iter.into_iter();

        for _ in 0..N {
            if let Some(item) = iter.next() {
                from.push(item);
            } else {
                break;
            }
        }

        Links { iter, from }
    }
}

impl<T, I, const N: usize> Iterator for Links<T, I, N>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    type Item = Link<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if N == 0 {
            return None;
        }

        let to = self.iter.next()?;
        let from: &[T; N] = self.from.as_slice().try_into().unwrap();
        let from = from.clone();

        self.from.rotate_left(1);
        self.from[N - 1] = to.clone();

        Some(Link::new(from, to))
    }
}

#[cfg(test)]
mod tests {
    use std::iter::empty;

    use super::super::types::Link;
    use super::{LinkIterator, Links};

    #[test]
    fn iterator() {
        let mut iter: Links<_, _, 3> = (0..6).into_iter().links();

        assert_eq!(iter.next(), Some(Link::new([0, 1, 2], 3)));
        assert_eq!(iter.next(), Some(Link::new([1, 2, 3], 4)));
        assert_eq!(iter.next(), Some(Link::new([2, 3, 4], 5)));
        assert!(iter.next().is_none());
    }

    #[test]
    fn exhausted_iterator() {
        let mut iter: Links<u32, _, 3> = empty().links();

        assert!(iter.next().is_none());
    }

    #[test]
    fn zero_length_window() {
        let mut iter: Links<_, _, 0> = (0..5).into_iter().links();

        assert!(iter.next().is_none());
    }

    #[test]
    fn window_longer_than_iterator() {
        let mut iter: Links<_, _, 10> = (0..5).into_iter().links();

        assert!(iter.next().is_none());
    }
}
