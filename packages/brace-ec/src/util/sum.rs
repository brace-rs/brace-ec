use num_traits::{CheckedAdd, Zero};

use super::iter::Iterable;

pub trait CheckedSum<T> {
    fn checked_sum(&self) -> Option<T>;
}

impl<T, I> CheckedSum<T> for I
where
    T: CheckedAdd + Zero,
    I: Iterable<Item = T>,
{
    fn checked_sum(&self) -> Option<T> {
        self.iter()
            .try_fold(T::zero(), |acc, value| acc.checked_add(value))
    }
}
