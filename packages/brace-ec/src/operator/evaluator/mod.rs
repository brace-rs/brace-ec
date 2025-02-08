pub mod count;
pub mod function;
pub mod hiff;

use std::error::Error;

use crate::individual::Individual;

pub trait Evaluator<T>
where
    T: Individual,
{
    type Error;

    fn evaluate<Rng>(&self, individual: &T, rng: &mut Rng) -> Result<T::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized;
}

pub trait DynEvaluator<I, E = Box<dyn Error + Send + Sync>>
where
    I: Individual,
{
    fn dyn_evaluate(&self, individual: &I, rng: &mut dyn rand::RngCore) -> Result<I::Fitness, E>;
}

impl<I, E, T> DynEvaluator<I, E> for T
where
    I: Individual,
    T: Evaluator<I, Error: Into<E>>,
{
    fn dyn_evaluate(&self, individual: &I, rng: &mut dyn rand::RngCore) -> Result<I::Fitness, E> {
        self.evaluate(individual, rng)
            .map(Into::into)
            .map_err(Into::into)
    }
}

impl<I, E> Evaluator<I> for Box<dyn DynEvaluator<I, E>>
where
    I: Individual,
{
    type Error = E;

    fn evaluate<Rng>(&self, individual: &I, mut rng: &mut Rng) -> Result<I::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_evaluate(individual, &mut rng)
    }
}
