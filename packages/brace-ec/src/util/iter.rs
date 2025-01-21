use std::convert::Infallible;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
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

pub trait ParIterable {
    type Item: Send;

    type ParIter<'a>: ParallelIterator<Item = &'a Self::Item>
    where
        Self: 'a;

    fn par_iter(&self) -> Self::ParIter<'_>;
}

impl<T, U> ParIterable for T
where
    T: ?Sized,
    U: Send,
    for<'a> &'a T: IntoParallelIterator<Item = &'a U>,
{
    type Item = U;

    type ParIter<'a>
        = <&'a T as IntoParallelIterator>::Iter
    where
        Self: 'a;

    fn par_iter(&self) -> Self::ParIter<'_> {
        self.into_par_iter()
    }
}

pub trait ParIterableMut: ParIterable {
    type ParIterMut<'a>: ParallelIterator<Item = &'a mut Self::Item>
    where
        Self: 'a;

    fn par_iter_mut(&mut self) -> Self::ParIterMut<'_>;
}

impl<T> ParIterableMut for T
where
    T: ParIterable + ?Sized,
    for<'a> &'a mut T: IntoParallelIterator<Item = &'a mut T::Item>,
{
    type ParIterMut<'a>
        = <&'a mut T as IntoParallelIterator>::Iter
    where
        Self: 'a;

    fn par_iter_mut(&mut self) -> Self::ParIterMut<'_> {
        self.into_par_iter()
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

impl<T, U, E> TryFromIterator<Result<T, E>> for Result<U, E>
where
    U: TryFromIterator<T>,
{
    type Error = U::Error;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = Result<T, E>>,
    {
        let res = iter
            .into_iter()
            .process_results(|iter| U::try_from_iter(iter));

        Ok(match res {
            Ok(res) => Ok(res?),
            Err(err) => Err(err),
        })
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TryFromIteratorError {
    #[error("not enough items in iterator")]
    NotEnough,
}
