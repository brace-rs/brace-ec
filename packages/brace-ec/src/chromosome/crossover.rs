use std::mem;
use std::ops::RangeBounds;

use crate::util::range::{bound_range, get_range, get_slice_range_mut};

use super::Chromosome;

pub trait Crossover: Chromosome {
    fn crossover_gene(&mut self, other: &mut Self, index: usize);

    fn crossover_segment<R>(&mut self, other: &mut Self, range: R)
    where
        R: RangeBounds<usize>,
    {
        for index in bound_range(get_range(range), 0..self.len()) {
            self.crossover_gene(other, index);
        }
    }
}

impl<T> Crossover for Vec<T> {
    fn crossover_gene(&mut self, other: &mut Self, index: usize) {
        self.as_mut_slice()
            .crossover_gene(other.as_mut_slice(), index);
    }

    fn crossover_segment<R>(&mut self, other: &mut Self, range: R)
    where
        R: RangeBounds<usize>,
    {
        self.as_mut_slice()
            .crossover_segment(other.as_mut_slice(), range);
    }
}

impl<T, const N: usize> Crossover for [T; N] {
    fn crossover_gene(&mut self, other: &mut Self, index: usize) {
        self.as_mut_slice()
            .crossover_gene(other.as_mut_slice(), index);
    }

    fn crossover_segment<R>(&mut self, other: &mut Self, range: R)
    where
        R: RangeBounds<usize>,
    {
        self.as_mut_slice()
            .crossover_segment(other.as_mut_slice(), range);
    }
}

impl<T> Crossover for [T] {
    fn crossover_gene(&mut self, other: &mut Self, index: usize) {
        if let (Some(lhs), Some(rhs)) = (self.get_mut(index), other.get_mut(index)) {
            mem::swap(lhs, rhs)
        }
    }

    fn crossover_segment<R>(&mut self, other: &mut Self, range: R)
    where
        R: RangeBounds<usize>,
    {
        let lhs = get_slice_range_mut(self, &range);
        let rhs = get_slice_range_mut(other, &range);

        lhs.swap_with_slice(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::Crossover;

    #[test]
    fn test_array() {
        let mut a = [0, 1, 2, 3, 4];
        let mut b = [5, 6, 7, 8, 9];

        a.crossover_gene(&mut b, 0);
        b.crossover_gene(&mut a, 4);

        assert_eq!(a, [5, 1, 2, 3, 9]);
        assert_eq!(b, [0, 6, 7, 8, 4]);

        a.crossover_segment(&mut b, 1..3);

        assert_eq!(a, [5, 6, 7, 3, 9]);
        assert_eq!(b, [0, 1, 2, 8, 4]);

        b.crossover_segment(&mut a, 0..=4);

        assert_eq!(a, [0, 1, 2, 8, 4]);
        assert_eq!(b, [5, 6, 7, 3, 9]);
    }

    #[test]
    fn test_vec() {
        let mut a = vec![0, 1, 2, 3, 4];
        let mut b = vec![5, 6, 7, 8, 9];

        a.crossover_gene(&mut b, 0);
        b.crossover_gene(&mut a, 4);

        assert_eq!(a, [5, 1, 2, 3, 9]);
        assert_eq!(b, [0, 6, 7, 8, 4]);

        a.crossover_segment(&mut b, 1..3);

        assert_eq!(a, [5, 6, 7, 3, 9]);
        assert_eq!(b, [0, 1, 2, 8, 4]);

        b.crossover_segment(&mut a, 0..=4);

        assert_eq!(a, [0, 1, 2, 8, 4]);
        assert_eq!(b, [5, 6, 7, 3, 9]);
    }

    #[test]
    fn test_slice() {
        let mut a = [0, 1, 2, 3, 4];
        let mut b = [5, 6, 7, 8, 9];

        a.as_mut_slice().crossover_gene(b.as_mut_slice(), 0);
        b.as_mut_slice().crossover_gene(a.as_mut_slice(), 4);

        assert_eq!(a, [5, 1, 2, 3, 9]);
        assert_eq!(b, [0, 6, 7, 8, 4]);

        a.as_mut_slice().crossover_segment(b.as_mut_slice(), 1..3);

        assert_eq!(a, [5, 6, 7, 3, 9]);
        assert_eq!(b, [0, 1, 2, 8, 4]);

        b.as_mut_slice().crossover_segment(a.as_mut_slice(), 0..=4);

        assert_eq!(a, [0, 1, 2, 8, 4]);
        assert_eq!(b, [5, 6, 7, 3, 9]);
    }
}
