pub mod sum;

use crate::core::fitness::{Fitness, FitnessMut};
use crate::core::population::Population;

use super::inspect::Inspect;
use super::repeat::Repeat;
use super::score::Score;
use super::scorer::function::Function;
use super::scorer::Scorer;
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

    fn score<S>(self, scorer: S) -> Score<Self, S>
    where
        S: Scorer<P::Individual, Score = <P::Individual as Fitness>::Value>,
        P::Individual: FitnessMut,
    {
        Score::new(self, scorer)
    }

    fn score_with<F, E>(self, scorer: F) -> Score<Self, Function<F>>
    where
        F: Fn(&P::Individual) -> Result<<P::Individual as Fitness>::Value, E>,
        P::Individual: FitnessMut,
    {
        self.score(Function::new(scorer))
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

    fn inspect<F>(self, inspector: F) -> Inspect<Self, F>
    where
        F: Fn(&Self::Output),
    {
        Inspect::new(self, inspector)
    }
}

pub trait DynRecombinator<P, O = Vec<<P as Population>::Individual>, E = Box<dyn std::error::Error>>
where
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
