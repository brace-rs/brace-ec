pub mod and;
pub mod best;
pub mod fill;
pub mod first;
pub mod generate;
pub mod hill_climb;
pub mod lexicase;
pub mod mutate;
pub mod random;
pub mod recombine;
pub mod take;
pub mod tournament;
pub mod windows;
pub mod worst;

use std::error::Error;

use crate::generation::Generation;
use crate::individual::Individual;
use crate::population::Population;

use self::and::And;
use self::fill::{Fill, ParFill};
use self::hill_climb::HillClimb;
use self::mutate::Mutate;
use self::recombine::Recombine;
use self::take::Take;
use self::windows::{ArrayWindows, ParArrayWindows, ParWindows, Windows};

use super::evaluate::Evaluate;
use super::evaluator::function::Function;
use super::evaluator::Evaluator;
use super::evolver::select::Select;
use super::inspect::Inspect;
use super::mutator::Mutator;
use super::recombinator::Recombinator;
use super::repeat::{Repeat, RepeatN};
use super::then::Then;

pub trait Selector<P>: Sized
where
    P: Population + ?Sized,
{
    type Output: Population<Individual = P::Individual>;
    type Error;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized;

    fn mutate<M>(self, mutator: M) -> Mutate<Self, M>
    where
        M: Mutator<P::Individual>,
    {
        Mutate::new(self, mutator)
    }

    fn recombine<R>(self, recombinator: R) -> Recombine<Self, R>
    where
        R: Recombinator<Self::Output>,
    {
        Recombine::new(self, recombinator)
    }

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

    fn hill_climb<M>(self, mutator: M, iterations: usize) -> HillClimb<Self, M>
    where
        M: Mutator<P::Individual>,
        Self: Selector<P, Output = [P::Individual; 1]>,
        P::Individual: Clone,
    {
        HillClimb::new(self, mutator, iterations)
    }

    fn evolver<G>(self) -> Select<Self, G>
    where
        G: Generation<Population = P>,
    {
        Select::new(self)
    }

    fn and<S>(self, selector: S) -> And<Self, S>
    where
        S: Selector<P>,
    {
        And::new(self, selector)
    }

    fn then<S>(self, selector: S) -> Then<Self, S>
    where
        S: Selector<Self::Output>,
    {
        Then::new(self, selector)
    }

    fn fill(self) -> Fill<Self> {
        Fill::new(self)
    }

    fn par_fill(self) -> ParFill<Self> {
        ParFill::new(self)
    }

    fn windows<T>(self, size: usize) -> Windows<Self, T>
    where
        T: AsRef<[P::Individual]> + ?Sized,
        Self: Selector<[P::Individual]>,
    {
        Windows::new(self, size)
    }

    fn par_windows<T>(self, size: usize) -> ParWindows<Self, T>
    where
        T: AsRef<[P::Individual]> + ?Sized,
        Self: Selector<[P::Individual]>,
    {
        ParWindows::new(self, size)
    }

    fn array_windows<const N: usize, T>(self) -> ArrayWindows<N, Self, T>
    where
        T: AsRef<[P::Individual]> + ?Sized,
        Self: Selector<[P::Individual; N]>,
    {
        ArrayWindows::new(self)
    }

    fn par_array_windows<const N: usize, T>(self) -> ParArrayWindows<N, Self, T>
    where
        T: AsRef<[P::Individual]> + ?Sized,
        Self: Selector<[P::Individual; N]>,
    {
        ParArrayWindows::new(self)
    }

    fn take<const N: usize>(self) -> Take<Self, N>
    where
        Self::Output: IntoIterator<Item = P::Individual>,
    {
        Take::new(self)
    }

    fn repeat(self, count: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
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
        Self: Sized,
    {
        Inspect::new(self, inspector)
    }
}

pub trait DynSelector<P, O = Vec<<P as Population>::Individual>, E = Box<dyn Error + Send + Sync>>
where
    P: Population + ?Sized,
    O: Population<Individual = P::Individual>,
{
    fn dyn_select(&self, population: &P, rng: &mut dyn rand::RngCore) -> Result<O, E>;
}

impl<P, O, E, T> DynSelector<P, O, E> for T
where
    P: Population + ?Sized,
    O: Population<Individual = P::Individual>,
    T: Selector<P, Output: Into<O>, Error: Into<E>>,
{
    fn dyn_select(&self, population: &P, rng: &mut dyn rand::RngCore) -> Result<O, E> {
        self.select(population, rng)
            .map(Into::into)
            .map_err(Into::into)
    }
}

impl<P, O, E> Selector<P> for Box<dyn DynSelector<P, O, E>>
where
    P: Population + ?Sized,
    O: Population<Individual = P::Individual>,
{
    type Output = O;
    type Error = E;

    fn select<Rng>(&self, population: &P, mut rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_select(population, &mut rng)
    }
}

impl<P, O, E> Selector<P> for Box<dyn DynSelector<P, O, E> + Send + Sync>
where
    P: Population + ?Sized,
    O: Population<Individual = P::Individual>,
{
    type Output = O;
    type Error = E;

    fn select<Rng>(&self, population: &P, mut rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_select(population, &mut rng)
    }
}
