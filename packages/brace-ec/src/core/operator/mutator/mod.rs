pub mod add;
pub mod invert;
pub mod noise;
pub mod rate;

use crate::core::individual::Individual;

use self::rate::Rate;

use super::inspect::Inspect;
use super::repeat::Repeat;
use super::score::Score;
use super::scorer::function::Function;
use super::scorer::Scorer;
use super::then::Then;

pub trait Mutator<T>: Sized
where
    T: Individual,
{
    type Error;

    fn mutate<Rng>(&self, individual: T, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized;

    fn score<S>(self, scorer: S) -> Score<Self, S>
    where
        S: Scorer<T, Score = T::Fitness>,
        T: Individual,
    {
        Score::new(self, scorer)
    }

    fn score_with<F, E>(self, scorer: F) -> Score<Self, Function<F>>
    where
        F: Fn(&T) -> Result<T::Fitness, E>,
        T: Individual,
    {
        self.score(Function::new(scorer))
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
