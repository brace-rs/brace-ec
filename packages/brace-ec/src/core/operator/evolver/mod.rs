pub mod select;

use crate::core::fitness::{Fitness, FitnessMut};
use crate::core::generation::Generation;
use crate::core::population::Population;

use super::inspect::Inspect;
use super::repeat::Repeat;
use super::score::Score;
use super::scorer::Scorer;

pub trait Evolver {
    type Generation: Generation;
    type Error;

    fn evolve(&self, generation: Self::Generation) -> Result<Self::Generation, Self::Error>;

    fn score<S>(self, scorer: S) -> Score<Self, S>
    where
        S: Scorer<
            Individual = <<Self::Generation as Generation>::Population as Population>::Individual,
            Score = <<<Self::Generation as Generation>::Population as Population>::Individual as Fitness>::Value,
        >,
        <<Self::Generation as Generation>::Population as Population>::Individual: FitnessMut,
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
        F: Fn(&Self::Generation),
        Self: Sized,
    {
        Inspect::new(self, inspector)
    }
}
