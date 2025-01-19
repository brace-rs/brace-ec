use std::convert::Infallible;

use thiserror::Error;

pub trait Iterable {
    type Item;

    type Iter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

impl<T, U> Iterable for T
where
    T: ?Sized,
    for<'a> &'a T: IntoIterator<Item = &'a U>,
{
    type Item = U;

    type Iter<'a>
        = <&'a T as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }
}

pub trait IterableMut: Iterable {
    type IterMut<'a>: Iterator<Item = &'a mut Self::Item>
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

impl<T> IterableMut for T
where
    T: Iterable + ?Sized,
    for<'a> &'a mut T: IntoIterator<Item = &'a mut T::Item>,
{
    type IterMut<'a>
        = <&'a mut T as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }
}

pub trait TryFromIterator<T>: Sized {
    type Error;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>;
}

impl<T, const N: usize> TryFromIterator<T> for [T; N] {
    type Error = TryFromIteratorError;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();

        match array_util::try_from_fn(|_| iter.next()) {
            Some(arr) => Ok(arr),
            None => Err(TryFromIteratorError::NotEnough),
        }
    }
}

impl<T> TryFromIterator<T> for Vec<T> {
    type Error = Infallible;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        Ok(iter.into_iter().collect())
    }
}

impl<T> TryFromIterator<T> for Option<T> {
    type Error = Infallible;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        Ok(iter.into_iter().next())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TryFromIteratorError {
    #[error("not enough items in iterator")]
    NotEnough,
}
