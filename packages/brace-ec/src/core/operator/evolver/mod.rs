pub mod select;

use crate::core::fitness::{Fitness, FitnessMut};
use crate::core::generation::Generation;
use crate::core::population::Population;

use super::inspect::Inspect;
use super::repeat::Repeat;
use super::score::Score;
use super::scorer::function::Function;
use super::scorer::Scorer;
use super::then::Then;

pub trait Evolver {
    type Generation: Generation;
    type Error;

    fn evolve(&self, generation: Self::Generation) -> Result<Self::Generation, Self::Error>;

    fn score<S>(self, scorer: S) -> Score<Self, S>
    where
        S: Scorer<
            <<Self::Generation as Generation>::Population as Population>::Individual,
            Score = <<<Self::Generation as Generation>::Population as Population>::Individual as Fitness>::Value,
        >,
        <<Self::Generation as Generation>::Population as Population>::Individual: FitnessMut,
        Self: Sized,
    {
        Score::new(self, scorer)
    }

    fn score_with<F, E>(
        self,
        scorer: F,
    ) -> Score<Self, Function<F>>
    where
        F: Fn(
            &<<Self::Generation as Generation>::Population as Population>::Individual,
        )
            -> Result<<<<Self::Generation as Generation>::Population as Population>::Individual as Fitness>::Value, E>,
        <<Self::Generation as Generation>::Population as Population>::Individual: FitnessMut,
        Self: Sized,
    {
        self.score(Function::new(scorer))
    }

    fn then<E>(self, evolver: E) -> Then<Self, E>
    where
        E: Evolver<Generation = Self::Generation>,
        Self: Sized,
    {
        Then::new(self, evolver)
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
