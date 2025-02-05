pub mod add;
pub mod each;
pub mod invert;
pub mod noise;
pub mod rate;

use crate::individual::Individual;
use crate::util::iter::IterableMut;

use self::each::Each;
use self::rate::Rate;

use super::evaluate::Evaluate;
use super::evaluator::function::Function;
use super::evaluator::Evaluator;
use super::inspect::Inspect;
use super::repeat::Repeat;
use super::then::Then;

pub trait Mutator<T>: Sized
where
    T: Individual,
{
    type Error;

    fn mutate<Rng>(&self, individual: T, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized;

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

    fn then<M>(self, mutator: M) -> Then<Self, M>
    where
        M: Mutator<T>,
    {
        Then::new(self, mutator)
    }

    fn rate(self, rate: f64) -> Rate<Self> {
        Rate::new(self, rate)
    }

    fn repeat(self, count: usize) -> Repeat<Self> {
        Repeat::new(self, count)
    }

    fn each<I>(self) -> Each<Self, I>
    where
        I: Individual<Genome: IterableMut<Item = T>>,
        T: Clone,
    {
        Each::new(self)
    }

    fn inspect<F>(self, inspector: F) -> Inspect<Self, F>
    where
        F: Fn(&T),
    {
        Inspect::new(self, inspector)
    }
}

pub trait DynMutator<I, E = Box<dyn std::error::Error>>
where
    I: Individual,
{
    fn dyn_mutate(&self, individual: I, rng: &mut dyn rand::RngCore) -> Result<I, E>;
}

impl<I, E, T> DynMutator<I, E> for T
where
    I: Individual,
    T: Mutator<I, Error: Into<E>>,
{
    fn dyn_mutate(&self, individual: I, rng: &mut dyn rand::RngCore) -> Result<I, E> {
        self.mutate(individual, rng).map_err(Into::into)
    }
}

impl<I, E> Mutator<I> for Box<dyn DynMutator<I, E>>
where
    I: Individual,
{
    type Error = E;

    fn mutate<Rng>(&self, individual: I, mut rng: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_mutate(individual, &mut rng)
    }
}
