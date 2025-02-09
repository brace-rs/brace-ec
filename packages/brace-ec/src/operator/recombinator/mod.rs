pub mod average;
pub mod point;
pub mod sum;
pub mod uniform;

use std::error::Error;

use crate::individual::Individual;
use crate::population::Population;

use super::evaluate::Evaluate;
use super::evaluator::function::Function;
use super::evaluator::Evaluator;
use super::inspect::Inspect;
use super::repeat::{Repeat, RepeatN};
use super::then::Then;

pub trait Recombinator<P>: Sized
where
    P: Population,
{
    type Output: Population<Individual = P::Individual>;
    type Error;

    fn recombine<Rng>(&self, parents: P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized;

    fn evaluate<S>(self, evaluator: S) -> Evaluate<Self, S>
    where
        S: Evaluator<P::Individual>,
    {
        Evaluate::new(self, evaluator)
    }

    fn evaluate_with<F, E>(self, evaluator: F) -> Evaluate<Self, Function<F>>
    where
        F: Fn(&P::Individual) -> Result<<P::Individual as Individual>::Fitness, E>,
    {
        self.evaluate(Function::new(evaluator))
    }

    fn then<R>(self, recombinator: R) -> Then<Self, R>
    where
        R: Recombinator<Self::Output>,
    {
        Then::new(self, recombinator)
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

    fn inspect<F>(self, inspector: F) -> Inspect<Self, F>
    where
        F: Fn(&Self::Output),
    {
        Inspect::new(self, inspector)
    }
}

pub trait DynRecombinator<
    P,
    O = Vec<<P as Population>::Individual>,
    E = Box<dyn Error + Send + Sync>,
> where
    P: Population,
    O: Population<Individual = P::Individual>,
{
    fn dyn_recombine(&self, population: P, rng: &mut dyn rand::RngCore) -> Result<O, E>;
}

impl<P, O, E, T> DynRecombinator<P, O, E> for T
where
    P: Population,
    O: Population<Individual = P::Individual>,
    T: Recombinator<P, Output: Into<O>, Error: Into<E>>,
{
    fn dyn_recombine(&self, population: P, rng: &mut dyn rand::RngCore) -> Result<O, E> {
        self.recombine(population, rng)
            .map(Into::into)
            .map_err(Into::into)
    }
}

impl<P, O, E> Recombinator<P> for Box<dyn DynRecombinator<P, O, E>>
where
    P: Population,
    O: Population<Individual = P::Individual>,
{
    type Output = O;
    type Error = E;

    fn recombine<Rng>(&self, population: P, mut rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_recombine(population, &mut rng)
    }
}

impl<P, O, E> Recombinator<P> for Box<dyn DynRecombinator<P, O, E> + Send + Sync>
where
    P: Population,
    O: Population<Individual = P::Individual>,
{
    type Output = O;
    type Error = E;

    fn recombine<Rng>(&self, population: P, mut rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_recombine(population, &mut rng)
    }
}
