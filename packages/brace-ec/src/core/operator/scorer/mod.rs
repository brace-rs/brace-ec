pub mod function;

use crate::core::individual::Individual;

pub trait Scorer<T>
where
    T: Individual,
{
    type Score: Ord;
    type Error;

    fn score<Rng>(&self, input: &T, rng: &mut Rng) -> Result<Self::Score, Self::Error>
    where
        Rng: rand::Rng + ?Sized;
}

pub trait DynScorer<I, S, E = Box<dyn std::error::Error>>
where
    I: Individual,
    S: Ord,
{
    fn dyn_score(&self, individual: &I, rng: &mut dyn rand::RngCore) -> Result<S, E>;
}

impl<I, S, E, T> DynScorer<I, S, E> for T
where
    I: Individual,
    S: Ord,
    T: Scorer<I, Score: Into<S>, Error: Into<E>>,
{
    fn dyn_score(&self, individual: &I, rng: &mut dyn rand::RngCore) -> Result<S, E> {
        self.score(individual, rng)
            .map(Into::into)
            .map_err(Into::into)
    }
}

impl<I, S, E> Scorer<I> for Box<dyn DynScorer<I, S, E>>
where
    I: Individual,
    S: Ord,
{
    type Score = S;
    type Error = E;

    fn score<Rng>(&self, individual: &I, mut rng: &mut Rng) -> Result<Self::Score, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_score(individual, &mut rng)
    }
}
