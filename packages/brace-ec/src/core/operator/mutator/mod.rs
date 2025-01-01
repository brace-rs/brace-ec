pub mod add;

use rand::Rng;

use crate::core::individual::Individual;

pub trait Mutator: Sized {
    type Individual: Individual;
    type Error;

    fn mutate<R>(
        &self,
        individual: Self::Individual,
        rng: &mut R,
    ) -> Result<Self::Individual, Self::Error>
    where
        R: Rng + ?Sized;
}
