use rand::Rng;
use thiserror::Error;

use crate::core::individual::Individual;
use crate::core::operator::recombinator::Recombinator;
use crate::core::population::Population;
use crate::linear::chromosome::Chromosome;
use crate::linear::crossover::Crossover;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct OnePointCrossover<P: Population>;

impl<I> Recombinator for OnePointCrossover<[I; 2]>
where
    I: Individual<Genome: Crossover>,
{
    type Parents = [I; 2];
    type Output = [I; 2];
    type Error = PointCrossoverError;

    fn recombine<R>(
        &self,
        [mut lhs, mut rhs]: Self::Parents,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>
    where
        R: Rng + ?Sized,
    {
        if lhs.genome().len() != rhs.genome().len() {
            return Err(PointCrossoverError::MixedLength);
        }

        if lhs.genome().len() < 1 {
            return Err(PointCrossoverError::TooManySegments);
        }

        let i = rng.gen_range(0..lhs.genome().len());

        lhs.genome_mut().crossover_segment(rhs.genome_mut(), 0..i);

        Ok([lhs, rhs])
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PointCrossoverError {
    #[error("unsupported crossover between genomes of different lengths")]
    MixedLength,
    #[error("crossover has more segments than genes")]
    TooManySegments,
}

#[cfg(test)]
mod tests {
    use crate::core::operator::recombinator::Recombinator;

    use super::{OnePointCrossover, PointCrossoverError};

    #[test]
    fn test_recombine_one_point() {
        let mut rng = rand::thread_rng();

        let lhs = [true, true, true, true, true];
        let rhs = [false, false, false, false, false];

        let [l, r] = OnePointCrossover.recombine([lhs, rhs], &mut rng).unwrap();

        assert!(l
            .iter()
            .all(|gene| lhs.contains(gene) || rhs.contains(gene)));
        assert!(r
            .iter()
            .all(|gene| lhs.contains(gene) || rhs.contains(gene)));
    }

    #[test]
    fn test_recombine_one_point_mixed_length() {
        let mut rng = rand::thread_rng();

        let lhs = vec![true, true];
        let rhs = vec![false, false, false];
        let res = OnePointCrossover.recombine([lhs, rhs], &mut rng);

        assert_eq!(res, Err(PointCrossoverError::MixedLength));
    }

    #[test]
    fn test_recombine_one_point_too_many_segments() {
        let mut rng = rand::thread_rng();

        let lhs = Vec::<i32>::new();
        let rhs = Vec::<i32>::new();
        let res = OnePointCrossover.recombine([lhs, rhs], &mut rng);

        assert_eq!(res, Err(PointCrossoverError::TooManySegments));
    }
}
