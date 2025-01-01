pub mod first;
pub mod mutate;
pub mod random;
pub mod recombine;

use rand::Rng;

use crate::core::population::Population;

use self::mutate::Mutate;
use self::recombine::Recombine;

use super::mutator::Mutator;
use super::recombinator::Recombinator;

pub trait Selector: Sized {
    type Population: Population;
    type Output: Population<Individual = <Self::Population as Population>::Individual>;
    type Error;

    fn select<R>(
        &self,
        population: &Self::Population,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>
    where
        R: Rng + ?Sized;

    fn mutate<M>(self, mutator: M) -> Mutate<Self, M>
    where
        M: Mutator<Individual = <Self::Population as Population>::Individual>,
    {
        Mutate::new(self, mutator)
    }

    fn recombine<R>(self, recombinator: R) -> Recombine<Self, R>
    where
        R: Recombinator<Parents = Self::Output>,
    {
        Recombine::new(self, recombinator)
    }
}
