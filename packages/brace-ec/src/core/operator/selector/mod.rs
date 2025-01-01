pub mod first;
pub mod mutate;
pub mod random;

use rand::Rng;

use crate::core::population::Population;

use self::mutate::Mutate;

use super::mutator::Mutator;

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
}
