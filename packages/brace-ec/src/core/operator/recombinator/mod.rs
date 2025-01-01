pub mod sum;

use rand::Rng;

use crate::core::population::Population;

pub trait Recombinator {
    type Parents: Population;
    type Output: Population<Individual = <Self::Parents as Population>::Individual>;
    type Error;

    fn recombine<R>(
        &self,
        parents: Self::Parents,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>
    where
        R: Rng + ?Sized;
}
