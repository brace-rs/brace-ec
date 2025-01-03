pub mod best;
pub mod first;
pub mod mutate;
pub mod random;
pub mod recombine;
pub mod tournament;

use rand::Rng;

use crate::core::fitness::{Fitness, FitnessMut};
use crate::core::population::Population;

use self::mutate::Mutate;
use self::recombine::Recombine;

use super::inspect::Inspect;
use super::mutator::Mutator;
use super::recombinator::Recombinator;
use super::repeat::Repeat;
use super::score::Score;
use super::scorer::Scorer;

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

    fn score<S>(self, scorer: S) -> Score<Self, S>
    where
        S: Scorer<
            Individual = <Self::Population as Population>::Individual,
            Score = <<Self::Population as Population>::Individual as Fitness>::Value,
        >,
        <Self::Population as Population>::Individual: FitnessMut,
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
