use num_traits::{CheckedAdd, Zero};

pub trait CheckedSum<T = Self>: Sized {
    fn checked_sum<I>(iter: I) -> Option<Self>
    where
        I: Iterator<Item = T>;
}

impl<T> CheckedSum<T> for T
where
    T: CheckedAdd + Zero,
{
    fn checked_sum<I>(mut iter: I) -> Option<Self>
    where
        I: Iterator<Item = T>,
    {
        iter.try_fold(T::zero(), |acc, value| acc.checked_add(&value))
    }
}

impl<'a, T> CheckedSum<&'a T> for T
where
    T: CheckedAdd + Zero,
{
    fn checked_sum<I>(mut iter: I) -> Option<Self>
    where
        I: Iterator<Item = &'a T>,
    {
        iter.try_fold(T::zero(), |acc, value| acc.checked_add(value))
    }
}
