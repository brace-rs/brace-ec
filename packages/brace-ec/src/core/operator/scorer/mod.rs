pub mod function;

use crate::core::individual::Individual;

pub trait Scorer<T>
where
    T: Individual,
{
    type Error;

    fn score<Rng>(&self, input: &T, rng: &mut Rng) -> Result<T::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized;
}

pub trait DynScorer<I, E = Box<dyn std::error::Error>>
where
    I: Individual,
{
    fn dyn_score(&self, individual: &I, rng: &mut dyn rand::RngCore) -> Result<I::Fitness, E>;
}

impl<I, E, T> DynScorer<I, E> for T
where
    I: Individual,
    T: Scorer<I, Error: Into<E>>,
{
    fn dyn_score(&self, individual: &I, rng: &mut dyn rand::RngCore) -> Result<I::Fitness, E> {
        self.score(individual, rng)
            .map(Into::into)
            .map_err(Into::into)
    }
}

impl<I, E> Scorer<I> for Box<dyn DynScorer<I, E>>
where
    I: Individual,
{
    type Error = E;

    fn score<Rng>(&self, individual: &I, mut rng: &mut Rng) -> Result<I::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_score(individual, &mut rng)
    }
}
