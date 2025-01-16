use std::marker::PhantomData;

use itertools::Itertools;
use thiserror::Error;

use crate::core::individual::Individual;
use crate::core::population::Population;

use super::Selector;

pub struct Windows<S, P>
where
    P: ?Sized,
{
    selector: S,
    size: usize,
    marker: PhantomData<fn() -> P>,
}

impl<S, P> Windows<S, P>
where
    P: ?Sized,
{
    pub fn new(selector: S, size: usize) -> Self {
        Self {
            selector,
            size,
            marker: PhantomData,
        }
    }
}

impl<P, S, T> Selector<P> for Windows<S, P>
where
    P: Population<Individual = T> + AsRef<[T]> + ?Sized,
    S: Selector<[T], Output: IntoIterator<Item = T>>,
    T: Individual,
{
    type Output = Vec<T>;
    type Error = WindowsError<S::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if self.size == 0 {
            return Err(WindowsError::Empty);
        }

        if population.len() < self.size {
            return Err(WindowsError::TooLarge);
        }

        population
            .as_ref()
            .windows(self.size)
            .map(|window| self.selector.select(window, rng))
            .flatten_ok()
            .collect::<Result<Vec<_>, _>>()
            .map_err(WindowsError::Select)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WindowsError<S> {
    #[error(transparent)]
    Select(S),
    #[error("window size is greater than population size")]
    TooLarge,
    #[error("window is empty")]
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::core::operator::mutator::add::Add;
    use crate::core::operator::recombinator::sum::Sum;
    use crate::core::operator::selector::best::Best;
    use crate::core::operator::selector::worst::Worst;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    use super::{Windows, WindowsError};

    #[test]
    fn test_select() {
        let population = [1, 2, 3, 4, 5];

        let a = population
            .select(Windows::new(Best, 2).mutate(Add(1)))
            .unwrap();
        let b = population
            .select(Windows::new(Best, 3).mutate(Add(1)))
            .unwrap();
        let c = population
            .select(Windows::new(Best, 4).mutate(Add(1)))
            .unwrap();
        let d = population
            .select(Windows::new(Best, 5).mutate(Add(1)))
            .unwrap();
        let e = population
            .select(Windows::new(Worst, 2).mutate(Add(1)))
            .unwrap();
        let f = population
            .select(Windows::new(Best.and(Worst), 2).mutate(Add(1)))
            .unwrap();
        let g = population
            .select(Windows::new(Best.and(Worst), 4).mutate(Add(1)))
            .unwrap();
        let h = population
            .select(Windows::new(Best.and(Worst).recombine(Sum), 4).mutate(Add(1)))
            .unwrap();
        let i = population.select(Windows::new(Best, 0));
        let j = population.select(Windows::new(Best, 6));

        assert_eq!(a, [3, 4, 5, 6]);
        assert_eq!(b, [4, 5, 6]);
        assert_eq!(c, [5, 6]);
        assert_eq!(d, [6]);
        assert_eq!(e, [2, 3, 4, 5]);
        assert_eq!(f, [3, 2, 4, 3, 5, 4, 6, 5]);
        assert_eq!(g, [5, 2, 6, 3]);
        assert_eq!(h, [6, 8]);
        assert_eq!(i, Err(WindowsError::Empty));
        assert_eq!(j, Err(WindowsError::TooLarge));
    }

    #[test]
    fn test_populations() {
        let a = [1, 2, 3, 4].select(Best.windows(1)).unwrap();
        let b = vec![1, 2, 3, 4].select(Best.windows(1)).unwrap();
        let c = [1, 2, 3, 4].as_slice().select(Best.windows(1)).unwrap();

        assert_eq!(a, [1, 2, 3, 4]);
        assert_eq!(b, [1, 2, 3, 4]);
        assert_eq!(c, [1, 2, 3, 4]);
    }
}
