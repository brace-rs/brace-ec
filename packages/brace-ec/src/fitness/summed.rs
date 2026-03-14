use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt::{self, Debug};
use std::iter::Sum;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Index};

use crate::util::iter::{Iterable, TryFromIterator};
use crate::util::sum::{Standard, TrySum};

use super::Fitness;

pub struct Summed<C, T = <C as Iterable>::Item, S = Standard>
where
    C: Iterable,
{
    value: C,
    total: T,
    strategy: PhantomData<fn() -> S>,
}

impl<C, T, S> Summed<C, T, S>
where
    C: Iterable<Item: Clone>,
    T: TrySum<C::Item, S, Error = Infallible>,
{
    pub fn new(value: C) -> Self {
        Self {
            total: T::try_sum(value.iter().cloned()).expect("infallible"),
            value,
            strategy: PhantomData,
        }
    }
}

impl<C, T, S> Summed<C, T, S>
where
    C: Iterable,
{
    pub fn value(&self) -> &C {
        &self.value
    }

    pub fn total(&self) -> &T {
        &self.total
    }
}

impl<C, T, S> Fitness for Summed<C, T, S>
where
    C: Iterable<Item: Clone> + Default,
    T: Ord + TrySum<C::Item, S, Error = Infallible>,
{
    fn nil() -> Self {
        Self::new(C::default())
    }
}

impl<C, T, S> Default for Summed<C, T, S>
where
    C: Iterable<Item: Clone> + Default,
    T: TrySum<C::Item, S, Error = Infallible>,
{
    fn default() -> Self {
        Self::new(C::default())
    }
}

impl<C, T, S> Clone for Summed<C, T, S>
where
    C: Iterable + Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            total: self.total.clone(),
            strategy: PhantomData,
        }
    }
}

impl<C, T, S> Debug for Summed<C, T, S>
where
    C: Iterable + Debug,
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Summed")
            .field("value", &self.value)
            .field("total", &self.total)
            .finish()
    }
}

impl<C, T, S> PartialEq for Summed<C, T, S>
where
    C: Iterable,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
    }
}

impl<C, T, S> Eq for Summed<C, T, S>
where
    C: Iterable,
    T: Eq,
{
}

impl<C, T, S> PartialOrd for Summed<C, T, S>
where
    C: Iterable,
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total.partial_cmp(&other.total)
    }
}

impl<C, T, S> Ord for Summed<C, T, S>
where
    C: Iterable,
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.total.cmp(&other.total)
    }
}

impl<C, T, S> Add<C::Item> for Summed<C, T, S>
where
    C: Iterable<Item: Clone + Into<T>> + Extend<C::Item>,
    T: Clone + TrySum<T, S, Error = Infallible>,
{
    type Output = Self;

    fn add(mut self, rhs: C::Item) -> Self::Output {
        self += rhs;
        self
    }
}

impl<C, T, S> AddAssign<C::Item> for Summed<C, T, S>
where
    C: Iterable<Item: Clone + Into<T>> + Extend<C::Item>,
    T: Clone + TrySum<T, S, Error = Infallible>,
{
    fn add_assign(&mut self, rhs: C::Item) {
        let iter = [self.total.clone(), rhs.clone().into()].into_iter();

        self.total = T::try_sum(iter).expect("infallible");
        self.value.extend(Some(rhs));
    }
}

impl<C, T, S> Sum<C::Item> for Summed<C, T, S>
where
    C: Iterable<Item: Clone> + FromIterator<C::Item>,
    T: TrySum<C::Item, S, Error = Infallible>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = C::Item>,
    {
        Self::from_iter(iter)
    }
}

impl<C, T, S> TrySum<C::Item, S> for Summed<C, T, S>
where
    C: Iterable<Item: Clone> + FromIterator<C::Item>,
    T: TrySum<C::Item, S>,
{
    type Error = T::Error;

    fn try_sum<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: Iterator<Item = C::Item>,
    {
        Self::try_from_iter(iter)
    }
}

impl<I, C, T, S> Index<I> for Summed<C, T, S>
where
    C: Iterable + Index<I>,
{
    type Output = C::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.value.index(index)
    }
}

impl<C, T, S> Iterable for Summed<C, T, S>
where
    C: Iterable,
{
    type Item = C::Item;
    type Iter<'a>
        = C::Iter<'a>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.value.iter()
    }
}

impl<C, T, S> TryFromIterator<C::Item> for Summed<C, T, S>
where
    C: Iterable<Item: Clone> + FromIterator<C::Item>,
    T: TrySum<C::Item, S>,
{
    type Error = T::Error;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = C::Item>,
    {
        let value = iter.into_iter().collect::<C>();
        let total = T::try_sum(value.iter().cloned())?;

        Ok(Self {
            value,
            total,
            strategy: PhantomData,
        })
    }
}

impl<C, T, S> FromIterator<C::Item> for Summed<C, T, S>
where
    C: Iterable<Item: Clone> + FromIterator<C::Item>,
    T: TrySum<C::Item, S, Error = Infallible>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = C::Item>,
    {
        Self::try_from_iter(iter).expect("infallible")
    }
}

impl<C, T, S> From<C> for Summed<C, T, S>
where
    C: Iterable<Item: Clone>,
    T: TrySum<C::Item, S, Error = Infallible>,
{
    fn from(value: C) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Summed;

    #[test]
    fn test_summed() {
        let a = Summed::<Vec<u8>>::from_iter([1, 2, 3]);
        let b = Summed::<Vec<u8>, u16>::from_iter([1, 2, 3]);
        let c = Summed::<Vec<u8>, u32>::from_iter([1, 2, 3]);
        let d = Summed::<Vec<u8>, u64>::from_iter([1, 2, 3]);
        let e = Summed::<Vec<u8>, u128>::from_iter([1, 2, 3]);
        let f = Summed::<Vec<u8>, i32>::from_iter([1, 2, 3]);

        assert_eq!(*a.total(), 6_u8);
        assert_eq!(*b.total(), 6_u16);
        assert_eq!(*c.total(), 6_u32);
        assert_eq!(*d.total(), 6_u64);
        assert_eq!(*e.total(), 6_u128);
        assert_eq!(*f.total(), 6_i32);
    }
}
