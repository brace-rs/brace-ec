use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Index};

use crate::util::iter::Iterable;

use super::Fitness;

pub struct Summed<T>
where
    T: Iterable,
{
    value: T,
    total: T::Item,
}

impl<T> Summed<T>
where
    T: Iterable<Item: for<'a> Sum<&'a T::Item>>,
{
    pub fn new(value: T) -> Self {
        Self {
            total: value.iter().sum(),
            value,
        }
    }
}

impl<T> Summed<T>
where
    T: Iterable,
{
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn total(&self) -> &T::Item {
        &self.total
    }
}

impl<T> Fitness for Summed<T>
where
    T: Iterable<Item: Ord + for<'a> Sum<&'a T::Item>> + Default,
{
    fn nil() -> Self {
        Self::new(T::default())
    }
}

impl<T> Default for Summed<T>
where
    T: Iterable<Item: for<'a> Sum<&'a T::Item>> + Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for Summed<T>
where
    T: Iterable<Item: Clone> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            total: self.total.clone(),
        }
    }
}

impl<T> Debug for Summed<T>
where
    T: Iterable<Item: Debug> + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Summed")
            .field("value", &self.value)
            .field("total", &self.total)
            .finish()
    }
}

impl<T> PartialEq for Summed<T>
where
    T: Iterable<Item: PartialEq>,
{
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
    }
}

impl<T> Eq for Summed<T> where T: Iterable<Item: Eq> {}

impl<T> PartialOrd for Summed<T>
where
    T: Iterable<Item: PartialOrd>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total.partial_cmp(&other.total)
    }
}

impl<T> Ord for Summed<T>
where
    T: Iterable<Item: Ord>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.total.cmp(&other.total)
    }
}

impl<T> Add<T::Item> for Summed<T>
where
    T: Iterable<Item: AddAssign + Clone> + Extend<T::Item>,
{
    type Output = Self;

    fn add(mut self, rhs: T::Item) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> AddAssign<T::Item> for Summed<T>
where
    T: Iterable<Item: AddAssign + Clone> + Extend<T::Item>,
{
    fn add_assign(&mut self, rhs: T::Item) {
        self.total += rhs.clone();
        self.value.extend(Some(rhs));
    }
}

impl<I, T> Index<I> for Summed<T>
where
    T: Iterable + Index<I>,
{
    type Output = T::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.value.index(index)
    }
}

impl<T> Iterable for Summed<T>
where
    T: Iterable,
{
    type Item = T::Item;
    type Iter<'a>
        = T::Iter<'a>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.value.iter()
    }
}

impl<T> FromIterator<T::Item> for Summed<T>
where
    T: FromIterator<T::Item> + Iterable<Item: for<'a> Sum<&'a T::Item>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T::Item>,
    {
        let value = iter.into_iter().collect::<T>();

        Self {
            total: value.iter().sum(),
            value,
        }
    }
}

impl<T> From<T> for Summed<T>
where
    T: Iterable<Item: for<'a> Sum<&'a T::Item>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
