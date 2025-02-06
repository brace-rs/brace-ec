use std::marker::PhantomData;

use num_traits::ToPrimitive;
use rand::distr::{Bernoulli, Distribution};
use thiserror::Error;

use crate::chromosome::Chromosome;
use crate::individual::Individual;
use crate::operator::mutator::Mutator;
use crate::util::iter::{Iterable, IterableMut};

pub struct Each<M, I> {
    mutator: M,
    marker: PhantomData<fn() -> I>,
}

impl<M, I> Each<M, I> {
    pub fn new(mutator: M) -> Self {
        Self {
            mutator,
            marker: PhantomData,
        }
    }
}

impl<M, I> Mutator<I> for Each<M, I>
where
    I: Individual<Genome: IterableMut<Item: Individual + Clone>>,
    M: Mutator<<I::Genome as Iterable>::Item>,
{
    type Error = M::Error;

    fn mutate<Rng>(&self, mut individual: I, rng: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        for item in individual.genome_mut().iter_mut() {
            *item = self.mutator.mutate(item.clone(), rng)?;
        }

        Ok(individual)
    }
}

pub struct EachReciprocalRate<M, I> {
    mutator: M,
    marker: PhantomData<fn() -> I>,
}

impl<M, I> EachReciprocalRate<M, I> {
    pub fn new(mutator: M) -> Self {
        Self {
            mutator,
            marker: PhantomData,
        }
    }
}

impl<I, M, G> Mutator<I> for EachReciprocalRate<M, I>
where
    I: Individual<Genome: Chromosome<Gene = G>>,
    M: Mutator<G>,
    G: Individual + Clone,
{
    type Error = EachReciprocalRateError<M::Error>;

    fn mutate<Rng>(&self, mut individual: I, rng: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let len = individual
            .genome()
            .len()
            .to_u32()
            .ok_or(EachReciprocalRateError::TooLarge)?;

        if len == 0 {
            return Ok(individual);
        }

        let distr = Bernoulli::from_ratio(1, len).expect("len greater than or equal to 1");

        for index in 0..individual.genome().len() {
            if distr.sample(rng) {
                let gene = individual
                    .genome_mut()
                    .gene_mut(index)
                    .expect("index less than length");

                *gene = self
                    .mutator
                    .mutate(gene.clone(), rng)
                    .map_err(EachReciprocalRateError::Mutate)?;
            }
        }

        Ok(individual)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EachReciprocalRateError<M> {
    #[error("chromosome length is too large")]
    TooLarge,
    #[error(transparent)]
    Mutate(M),
}

#[cfg(test)]
mod tests {
    use crate::individual::Individual;
    use crate::operator::mutator::add::Add;
    use crate::operator::mutator::Mutator;

    #[test]
    fn test_mutate_each() {
        let mut rng = rand::rng();

        let a = Add(1).rate(1.0).each().mutate([1, 2, 3], &mut rng).unwrap();
        let b = Add(1).rate(0.0).each().mutate([1, 2, 3], &mut rng).unwrap();
        let c = Add(1)
            .rate(1.0)
            .each()
            .rate(0.0)
            .mutate([1, 2, 3], &mut rng)
            .unwrap();
        let d = Add(1)
            .rate(1.0)
            .each()
            .rate(1.0)
            .each()
            .rate(1.0)
            .mutate([[1, 2], [2, 3], [3, 4]], &mut rng)
            .unwrap();

        assert_eq!(a, [2, 3, 4]);
        assert_eq!(b, [1, 2, 3]);
        assert_eq!(c, [1, 2, 3]);
        assert_eq!(d, [[2, 3], [3, 4], [4, 5]]);
    }

    #[test]
    fn test_mutate_each_reciprocal_rate() {
        let a = [1, 2].mutated(Add(1).each_reciprocal_rate()).unwrap();
        let b = [1, 2]
            .mutated(Add(1).each_reciprocal_rate().rate(0.0))
            .unwrap();

        assert!(a == [1, 2] || a == [2, 2] || a == [1, 3] || a == [2, 3]);
        assert_eq!(b, [1, 2]);
    }
}
