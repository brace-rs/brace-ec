pub mod select;

use crate::core::generation::Generation;

use super::repeat::Repeat;

pub trait Evolver {
    type Generation: Generation;
    type Error;

    fn evolve(&self, generation: Self::Generation) -> Result<Self::Generation, Self::Error>;

    fn repeat(self, count: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, count)
    }
}
