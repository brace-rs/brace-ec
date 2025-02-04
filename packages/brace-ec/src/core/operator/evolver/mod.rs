pub mod limit;
pub mod select;

use crate::core::generation::Generation;
use crate::core::individual::Individual;
use crate::core::population::Population;

use self::limit::Limit;

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
        S: Scorer<<G::Population as Population>::Individual>,
    {
        Score::new(self, scorer)
    }

    fn score_with<F, E>(self, scorer: F) -> Score<Self, Function<F>>
    where
        F: Fn(
            &<G::Population as Population>::Individual,
        )
            -> Result<<<G::Population as Population>::Individual as Individual>::Fitness, E>,
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

    fn limit(self, generation: G::Id) -> Limit<G, Self> {
        Limit::new(self, generation)
    }

    fn inspect<F>(self, inspector: F) -> Inspect<Self, F>
    where
        F: Fn(&G),
    {
        Inspect::new(self, inspector)
    }
}

pub trait DynEvolver<G, E = Box<dyn std::error::Error>>
where
    G: Generation,
{
    fn dyn_evolve(&self, generation: G, rng: &mut dyn rand::RngCore) -> Result<G, E>;
}

impl<G, E, T> DynEvolver<G, E> for T
where
    G: Generation,
    T: Evolver<G, Error: Into<E>>,
{
    fn dyn_evolve(&self, generation: G, rng: &mut dyn rand::RngCore) -> Result<G, E> {
        self.evolve(generation, rng).map_err(Into::into)
    }
}

impl<G, E> Evolver<G> for Box<dyn DynEvolver<G, E>>
where
    G: Generation,
{
    type Error = E;

    fn evolve<Rng>(&self, individual: G, mut rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_evolve(individual, &mut rng)
    }
}
