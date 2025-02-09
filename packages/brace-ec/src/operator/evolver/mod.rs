pub mod limit;
pub mod select;

use std::error::Error;

use crate::generation::Generation;
use crate::individual::Individual;
use crate::population::Population;

use self::limit::Limit;

use super::evaluate::Evaluate;
use super::evaluator::function::Function;
use super::evaluator::Evaluator;
use super::inspect::Inspect;
use super::repeat::{Repeat, RepeatN};
use super::then::Then;

pub trait Evolver<G>: Sized
where
    G: Generation,
{
    type Error;

    fn evolve<Rng>(&self, generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized;

    fn evaluate<S>(self, evaluator: S) -> Evaluate<Self, S>
    where
        S: Evaluator<<G::Population as Population>::Individual>,
    {
        Evaluate::new(self, evaluator)
    }

    fn evaluate_with<F, E>(self, evaluator: F) -> Evaluate<Self, Function<F>>
    where
        F: Fn(
            &<G::Population as Population>::Individual,
        )
            -> Result<<<G::Population as Population>::Individual as Individual>::Fitness, E>,
        Self: Sized,
    {
        self.evaluate(Function::new(evaluator))
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

    fn repeat_n<const N: usize>(self) -> RepeatN<N, Self> {
        RepeatN::new(self)
    }

    fn twice(self) -> RepeatN<2, Self> {
        self.repeat_n()
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

pub trait DynEvolver<G, E = Box<dyn Error + Send + Sync>>
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

impl<G, E> Evolver<G> for Box<dyn DynEvolver<G, E> + Send + Sync>
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
