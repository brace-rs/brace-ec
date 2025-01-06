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

pub trait Evolver<G>: Sized
where
    G: Generation,
{
    type Error;

    fn evolve<Rng>(&self, generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized;

    fn score<S>(self, scorer: S) -> Score<Self, S>
    where
        S: Scorer<
            <G::Population as Population>::Individual,
            Score = <<G::Population as Population>::Individual as Fitness>::Value,
        >,
        <G::Population as Population>::Individual: FitnessMut,
    {
        Score::new(self, scorer)
    }

    fn score_with<F, E>(self, scorer: F) -> Score<Self, Function<F>>
    where
        F: Fn(
            &<G::Population as Population>::Individual,
        ) -> Result<<<G::Population as Population>::Individual as Fitness>::Value, E>,
        <G::Population as Population>::Individual: FitnessMut,
        Self: Sized,
    {
        self.score(Function::new(scorer))
    }

    fn then<E>(self, evolver: E) -> Then<Self, E>
    where
        E: Evolver<G>,
    {
        Then::new(self, evolver)
    }

    fn repeat(self, count: usize) -> Repeat<Self> {
        Repeat::new(self, count)
    }

    fn inspect<F>(self, inspector: F) -> Inspect<Self, F>
    where
        F: Fn(&G),
    {
        Inspect::new(self, inspector)
    }
}
