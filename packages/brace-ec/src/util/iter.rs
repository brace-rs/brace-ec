use std::convert::Infallible;

use thiserror::Error;

pub trait Iterable {
    type Item;

    type Iter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

impl<const N: usize, T> Iterable for [T; N] {
    type Item = T;

    type Iter<'a>
        = std::slice::Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }
}

impl<T> Iterable for Vec<T> {
    type Item = T;

    type Iter<'a>
        = std::slice::Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        (**self).iter()
    }
}

impl<T> Iterable for Option<T> {
    type Item = T;

    type Iter<'a>
        = std::option::Iter<'a, T>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

pub trait IterableMut: Iterable {
    type IterMut<'a>: Iterator<Item = &'a mut Self::Item>
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

impl<const N: usize, T> IterableMut for [T; N] {
    type IterMut<'a>
        = std::slice::IterMut<'a, T>
    where
        T: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.as_mut_slice().iter_mut()
    }
}

impl<T> IterableMut for Vec<T> {
    type IterMut<'a>
        = std::slice::IterMut<'a, T>
    where
        T: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        (**self).iter_mut()
    }
}

impl<T> IterableMut for Option<T> {
    type IterMut<'a>
        = std::option::IterMut<'a, T>
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
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
