pub mod add;
pub mod invert;
pub mod rate;

use rand::Rng;

use crate::core::individual::Individual;

use self::rate::Rate;

use super::inspect::Inspect;
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

    fn rate(self, rate: f64) -> Rate<Self>
    where
        Self: Sized,
    {
        Rate::new(self, rate)
    }

    fn repeat(self, count: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, count)
    }

    fn inspect<F>(self, inspector: F) -> Inspect<Self, F>
    where
        F: Fn(&Self::Individual),
        Self: Sized,
    {
        Inspect::new(self, inspector)
    }
}
