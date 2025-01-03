pub mod sum;

use rand::Rng;

use crate::core::fitness::{Fitness, FitnessMut};
use crate::core::population::Population;

use super::inspect::Inspect;
use super::repeat::Repeat;
use super::score::Score;
use super::scorer::Scorer;

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

    fn score<S>(self, scorer: S) -> Score<Self, S>
    where
        S: Scorer<
            Individual = <Self::Parents as Population>::Individual,
            Score = <<Self::Parents as Population>::Individual as Fitness>::Value,
        >,
        <Self::Parents as Population>::Individual: FitnessMut,
        Self: Sized,
    {
        Score::new(self, scorer)
    }

    fn repeat(self, count: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, count)
    }

    fn inspect<F>(self, inspector: F) -> Inspect<Self, F>
    where
        F: Fn(&Self::Output),
        Self: Sized,
    {
        Inspect::new(self, inspector)
    }
}
