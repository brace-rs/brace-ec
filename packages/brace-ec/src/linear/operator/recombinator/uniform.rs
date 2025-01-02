use std::marker::PhantomData;

use rand::Rng;
use thiserror::Error;

use crate::core::individual::Individual;
use crate::core::operator::recombinator::Recombinator;
use crate::core::population::Population;
use crate::linear::chromosome::Chromosome;
use crate::linear::crossover::Crossover;

pub struct UniformCrossover<P: Population> {
    probability: f64,
    marker: PhantomData<fn() -> P>,
}

impl<P> UniformCrossover<P>
where
    P: Population,
{
    pub fn new(probability: f64) -> Self {
        Self {
            probability,
            marker: PhantomData,
        }
    }
}

impl<I> Recombinator for UniformCrossover<[I; 2]>
where
    I: Individual<Genome: Crossover>,
{
    type Parents = [I; 2];
    type Output = [I; 2];
    type Error = UniformCrossoverError;

    fn recombine<R>(
        &self,
        [mut lhs, mut rhs]: Self::Parents,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>
    where
        R: Rng + ?Sized,
    {
        if lhs.genome().len() != rhs.genome().len() {
            return Err(UniformCrossoverError::MixedLength);
        }

        for index in 0..lhs.genome().len() {
            if rng.gen_bool(self.probability) {
                lhs.genome_mut().crossover_gene(rhs.genome_mut(), index);
            }
        }

        Ok([lhs, rhs])
    }
}

impl<P> Default for UniformCrossover<P>
where
    P: Population,
{
    fn default() -> Self {
        Self {
            probability: 0.5,
            marker: PhantomData,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UniformCrossoverError {
    #[error("unsupported crossover between genomes of different lengths")]
    MixedLength,
}

#[cfg(test)]
mod tests {
    use crate::core::operator::recombinator::Recombinator;

    use super::{UniformCrossover, UniformCrossoverError};

    #[test]
    fn test_recombine() {
        let mut rng = rand::thread_rng();

        let lhs = [true, true, true, true, true];
        let rhs = [false, false, false, false, false];

        let a = UniformCrossover::new(0.0)
            .recombine([lhs, rhs], &mut rng)
            .unwrap();
        let b = UniformCrossover::new(1.0)
            .recombine([lhs, rhs], &mut rng)
            .unwrap();

        assert_eq!(a, [lhs, rhs]);
        assert_eq!(b, [rhs, lhs]);
    }

    #[test]
    fn test_recombine_mixed() {
        let mut rng = rand::thread_rng();

        let lhs = vec![true, true];
        let rhs = vec![false, false, false];

        let a = UniformCrossover::new(0.0).recombine([lhs.clone(), rhs.clone()], &mut rng);
        let b = UniformCrossover::new(1.0).recombine([lhs, rhs], &mut rng);

        assert_eq!(a, Err(UniformCrossoverError::MixedLength));
        assert_eq!(b, Err(UniformCrossoverError::MixedLength));
    }
}
