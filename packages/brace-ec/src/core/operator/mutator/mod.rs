pub mod add;
pub mod invert;

use rand::Rng;

use crate::core::individual::Individual;

use super::repeat::Repeat;

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

    fn repeat(self, count: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, count)
    }
}
