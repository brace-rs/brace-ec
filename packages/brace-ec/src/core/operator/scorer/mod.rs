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
