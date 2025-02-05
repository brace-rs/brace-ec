use std::marker::PhantomData;

use rand::seq::IteratorRandom;
use thiserror::Error;

use crate::chromosome::crossover::Crossover;
use crate::chromosome::Chromosome;
use crate::individual::Individual;
use crate::operator::recombinator::Recombinator;
use crate::population::Population;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct OnePointCrossover<P: Population>;

impl<I> Recombinator<[I; 2]> for OnePointCrossover<[I; 2]>
where
    I: Individual<Genome: Crossover>,
{
    type Output = [I; 2];
    type Error = PointCrossoverError;

    fn recombine<Rng>(
        &self,
        [mut lhs, mut rhs]: [I; 2],
        rng: &mut Rng,
    ) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if lhs.genome().len() != rhs.genome().len() {
            return Err(PointCrossoverError::MixedLength);
        }

        if lhs.genome().len() < 1 {
            return Err(PointCrossoverError::TooManySegments);
        }

        let point = rng.random_range(0..lhs.genome().len());

        lhs.genome_mut()
            .crossover_segment(rhs.genome_mut(), point..);

        Ok([lhs, rhs])
    }
}

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct TwoPointCrossover<P: Population>;

impl<I> Recombinator<[I; 2]> for TwoPointCrossover<[I; 2]>
where
    I: Individual<Genome: Crossover>,
{
    type Output = [I; 2];
    type Error = PointCrossoverError;

    fn recombine<Rng>(
        &self,
        [mut lhs, mut rhs]: [I; 2],
        rng: &mut Rng,
    ) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if lhs.genome().len() != rhs.genome().len() {
            return Err(PointCrossoverError::MixedLength);
        }

        if lhs.genome().len() < 2 {
            return Err(PointCrossoverError::TooManySegments);
        }

        let a = rng.random_range(0..lhs.genome().len());
        let b = rng.random_range(0..lhs.genome().len());

        lhs.genome_mut()
            .crossover_segment(rhs.genome_mut(), a.min(b)..b.max(a));

        Ok([lhs, rhs])
    }
}

pub struct MultiPointCrossover<P: Population> {
    count: usize,
    marker: PhantomData<fn() -> P>,
}

impl<P> MultiPointCrossover<P>
where
    P: Population,
{
    pub fn new(count: usize) -> Self {
        Self {
            count,
            marker: PhantomData,
        }
    }
}

impl<I> Recombinator<[I; 2]> for MultiPointCrossover<[I; 2]>
where
    I: Individual<Genome: Crossover>,
{
    type Output = [I; 2];
    type Error = PointCrossoverError;

    fn recombine<Rng>(&self, parents: [I; 2], rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        match self.count {
            0 => Ok(parents),
            1 => OnePointCrossover.recombine(parents, rng),
            2 => TwoPointCrossover.recombine(parents, rng),
            n => {
                let [mut lhs, mut rhs] = parents;

                if lhs.genome().len() != rhs.genome().len() {
                    return Err(PointCrossoverError::MixedLength);
                }

                if lhs.genome().len() < n {
                    return Err(PointCrossoverError::TooManySegments);
                }

                let mut points = (0..lhs.genome().len()).choose_multiple(rng, n);

                points.sort();

                for point in points {
                    lhs.genome_mut()
                        .crossover_segment(rhs.genome_mut(), point..);
                }

                Ok([lhs, rhs])
            }
        }
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
    use crate::operator::recombinator::Recombinator;

    use super::{MultiPointCrossover, OnePointCrossover, PointCrossoverError, TwoPointCrossover};

    #[test]
    fn test_recombine_one_point() {
        let mut rng = rand::rng();

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
        let mut rng = rand::rng();

        let lhs = vec![true, true];
        let rhs = vec![false, false, false];
        let res = OnePointCrossover.recombine([lhs, rhs], &mut rng);

        assert_eq!(res, Err(PointCrossoverError::MixedLength));
    }

    #[test]
    fn test_recombine_one_point_too_many_segments() {
        let mut rng = rand::rng();

        let lhs = Vec::<i32>::new();
        let rhs = Vec::<i32>::new();
        let res = OnePointCrossover.recombine([lhs, rhs], &mut rng);

        assert_eq!(res, Err(PointCrossoverError::TooManySegments));
    }

    #[test]
    fn test_recombine_two_point() {
        let mut rng = rand::rng();

        let lhs = [true, true, true, true, true];
        let rhs = [false, false, false, false, false];

        let [l, r] = TwoPointCrossover.recombine([lhs, rhs], &mut rng).unwrap();

        assert!(l
            .iter()
            .all(|gene| lhs.contains(gene) || rhs.contains(gene)));
        assert!(r
            .iter()
            .all(|gene| lhs.contains(gene) || rhs.contains(gene)));
    }

    #[test]
    fn test_recombine_two_point_mixed_length() {
        let mut rng = rand::rng();

        let lhs = vec![true, true];
        let rhs = vec![false, false, false];
        let res = TwoPointCrossover.recombine([lhs, rhs], &mut rng);

        assert_eq!(res, Err(PointCrossoverError::MixedLength));
    }

    #[test]
    fn test_recombine_two_point_too_many_segments() {
        let mut rng = rand::rng();

        let lhs = vec![1];
        let rhs = vec![1];
        let res = TwoPointCrossover.recombine([lhs, rhs], &mut rng);

        assert_eq!(res, Err(PointCrossoverError::TooManySegments));
    }

    #[test]
    fn test_recombine_multi_point() {
        let mut rng = rand::rng();

        let lhs = [true, true, true, true, true, true, true];
        let rhs = [false, false, false, false, false, false, false];

        let [l, r] = MultiPointCrossover::new(3)
            .recombine([lhs, rhs], &mut rng)
            .unwrap();

        assert!(l
            .iter()
            .all(|gene| lhs.contains(gene) || rhs.contains(gene)));
        assert!(r
            .iter()
            .all(|gene| lhs.contains(gene) || rhs.contains(gene)));
    }

    #[test]
    fn test_recombine_multi_point_mixed_length() {
        let mut rng = rand::rng();

        let lhs = vec![true, true, true, true];
        let rhs = vec![false, false, false, false, false];
        let res = MultiPointCrossover::new(3).recombine([lhs, rhs], &mut rng);

        assert_eq!(res, Err(PointCrossoverError::MixedLength));
    }

    #[test]
    fn test_recombine_multi_point_too_many_segments() {
        let mut rng = rand::rng();

        let lhs = [true, true, true, true, true];
        let rhs = [false, false, false, false, false];
        let res = MultiPointCrossover::new(6).recombine([lhs, rhs], &mut rng);

        assert_eq!(res, Err(PointCrossoverError::TooManySegments));
    }

    #[test]
    fn test_recombine_multi_point_uniform() {
        let mut rng = rand::rng();

        let lhs = [true, true, true, true, true, true];
        let rhs = [false, false, false, false, false, false];
        let [l, r] = MultiPointCrossover::new(6)
            .recombine([lhs, rhs], &mut rng)
            .unwrap();

        assert_eq!(l, [false, true, false, true, false, true]);
        assert_eq!(r, [true, false, true, false, true, false]);
    }
}
