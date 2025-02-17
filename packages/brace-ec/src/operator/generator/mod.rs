pub mod counter;
pub mod populate;
pub mod random;
pub mod search;

use std::error::Error;

use crate::individual::Individual;
use crate::population::Population;
use crate::util::iter::TryFromIterator;

use self::populate::Populate;
use self::search::{ParSearch, Search};

use super::evaluate::Evaluate;
use super::evaluator::function::Function;
use super::evaluator::Evaluator;
use super::selector::generate::Generate;

pub trait Generator<T>: Sized {
    type Error;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized;

    fn populate<P>(self, size: usize) -> Populate<Self, P>
    where
        P: Population<Individual = T> + TryFromIterator<T>,
    {
        Populate::new(self, size)
    }

    fn evaluate<S>(self, evaluator: S) -> Evaluate<Self, S>
    where
        S: Evaluator<T>,
        T: Individual,
    {
        Evaluate::new(self, evaluator)
    }

    fn evaluate_with<F, E>(self, evaluator: F) -> Evaluate<Self, Function<F>>
    where
        F: Fn(&T) -> Result<T::Fitness, E>,
        T: Individual,
    {
        self.evaluate(Function::new(evaluator))
    }

    fn search(self, iterations: usize) -> Search<Self>
    where
        T: Individual,
    {
        Search::new(self, iterations)
    }

    fn par_search(self, iterations: usize) -> ParSearch<Self>
    where
        T: Individual + Send,
        Self: Sync,
        Self::Error: Send,
    {
        ParSearch::new(self, iterations)
    }

    fn selector<P>(self) -> Generate<Self, P>
    where
        P: Population<Individual = T> + ?Sized,
    {
        Generate::new(self)
    }
}

pub trait DynGenerator<T, E = Box<dyn Error + Send + Sync>> {
    fn dyn_generate(&self, rng: &mut dyn rand::RngCore) -> Result<T, E>;
}

impl<T, E, G> DynGenerator<T, E> for G
where
    G: Generator<T, Error: Into<E>>,
{
    fn dyn_generate(&self, rng: &mut dyn rand::RngCore) -> Result<T, E> {
        self.generate(rng).map_err(Into::into)
    }
}

impl<T, E> Generator<T> for Box<dyn DynGenerator<T, E>> {
    type Error = E;

    fn generate<Rng>(&self, mut rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_generate(&mut rng)
    }
}

impl<T, E> Generator<T> for Box<dyn DynGenerator<T, E> + Send + Sync> {
    type Error = E;

    fn generate<Rng>(&self, mut rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_generate(&mut rng)
    }
}
