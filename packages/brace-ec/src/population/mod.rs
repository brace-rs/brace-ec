use crate::util::iter::{Iterable, IterableMut, ParIterable, ParIterableMut};

use super::individual::Individual;
use super::operator::recombinator::Recombinator;
use super::operator::selector::Selector;

pub trait Population {
    type Individual: Individual;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn select<S>(&self, selector: S) -> Result<S::Output, S::Error>
    where
        S: Selector<Self>,
    {
        selector.select(self, &mut rand::rng())
    }

    fn recombined<R>(self, recombinator: R) -> Result<R::Output, R::Error>
    where
        R: Recombinator<Self>,
        Self: Sized,
    {
        recombinator.recombine(self, &mut rand::rng())
    }
}

impl<T> Population for [T]
where
    T: Individual,
{
    type Individual = T;

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> Population for [T; N]
where
    T: Individual,
{
    type Individual = T;

    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<T> Population for Vec<T>
where
    T: Individual,
{
    type Individual = T;

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Population for Option<T>
where
    T: Individual,
{
    type Individual = T;

    fn len(&self) -> usize {
        match self {
            Some(_) => 1,
            None => 0,
        }
    }
}

pub trait IterablePopulation: Population + Iterable<Item = Self::Individual> {}

impl<T> IterablePopulation for T where T: Population + Iterable<Item = Self::Individual> + ?Sized {}

pub trait IterableMutPopulation: Population + IterableMut<Item = Self::Individual> {}

impl<T> IterableMutPopulation for T where
    T: Population + IterableMut<Item = Self::Individual> + ?Sized
{
}

pub trait ParIterablePopulation: Population + ParIterable<Item = Self::Individual> {}

impl<T> ParIterablePopulation for T where
    T: Population + ParIterable<Item = Self::Individual> + ?Sized
{
}

pub trait ParIterableMutPopulation: Population + ParIterableMut<Item = Self::Individual> {}

impl<T> ParIterableMutPopulation for T where
    T: Population + ParIterableMut<Item = Self::Individual> + ?Sized
{
}

pub trait ToOwnedPopulation:
    Population + ToOwned<Owned: Population<Individual = Self::Individual>>
{
}

impl<T> ToOwnedPopulation for T where
    T: Population + ToOwned<Owned: Population<Individual = Self::Individual>> + ?Sized
{
}

#[cfg(test)]
mod tests {
    use crate::individual::Individual;
    use crate::util::iter::Iterable;

    use super::{IterablePopulation, Population};

    fn erase<P: Population>(population: P) -> impl Population {
        population
    }

    fn erase_iter<I, P>(population: P) -> impl IterablePopulation<Individual = I>
    where
        I: Individual,
        P: IterablePopulation<Individual = I>,
    {
        population
    }

    #[test]
    fn test_population_array() {
        let population = erase([[0, 0]]);

        assert!(!population.is_empty());
        assert_eq!(population.len(), 1);

        let population = erase([[0, 0], [1, 1]]);

        assert!(!population.is_empty());
        assert_eq!(population.len(), 2);

        let population = erase_iter([[0], [1], [2]]);

        let mut iter = population.iter();

        assert_eq!(iter.next(), Some(&[0]));
        assert_eq!(iter.next(), Some(&[1]));
        assert_eq!(iter.next(), Some(&[2]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_population_vec() {
        let population = erase(Vec::<[u32; 2]>::new());

        assert!(population.is_empty());
        assert_eq!(population.len(), 0);

        let population = erase(vec![[0, 0]]);

        assert!(!population.is_empty());
        assert_eq!(population.len(), 1);

        let population = erase(vec![[0, 0], [1, 1]]);

        assert!(!population.is_empty());
        assert_eq!(population.len(), 2);

        let population = erase_iter(vec![[0], [1], [2]]);

        let mut iter = population.iter();

        assert_eq!(iter.next(), Some(&[0]));
        assert_eq!(iter.next(), Some(&[1]));
        assert_eq!(iter.next(), Some(&[2]));
        assert_eq!(iter.next(), None);
    }
}
