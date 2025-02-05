use num_traits::{CheckedDiv, FromPrimitive};
use thiserror::Error;

use crate::individual::Individual;
use crate::population::IterablePopulation;
use crate::util::sum::CheckedSum;

use super::Recombinator;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Average<P: IterablePopulation>;

impl<P, G> Recombinator<P> for Average<P>
where
    P: IterablePopulation<Individual: Individual<Genome = G> + From<G>>,
    G: for<'a> CheckedSum<&'a G> + CheckedDiv + FromPrimitive,
{
    type Output = [P::Individual; 1];
    type Error = AverageError;

    fn recombine<Rng>(&self, parents: P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if parents.len() == 0 {
            return Err(AverageError::Empty);
        }

        let Some(len) = G::from_usize(parents.len()) else {
            return Err(AverageError::Unrepresentable);
        };

        let genome = G::checked_sum(parents.iter().map(Individual::genome))
            .ok_or(AverageError::Wrap)?
            .checked_div(&len)
            .ok_or(AverageError::Wrap)?;

        Ok([genome.into()])
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AverageError {
    #[error("summation would wrap")]
    Wrap,
    #[error("population length could not be represented as genome")]
    Unrepresentable,
    #[error("empty population")]
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::individual::scored::Scored;
    use crate::operator::recombinator::Recombinator;

    use super::{Average, AverageError};

    #[test]
    fn test_recombine() {
        let mut rng = rand::rng();

        let a = Average.recombine([0, 0], &mut rng);
        let b = Average.recombine([1, 1, 1], &mut rng);
        let c = Average.recombine([1, i32::MAX], &mut rng);
        let d = Average.recombine([Scored::new(3, 0), Scored::new(4, 0)], &mut rng);
        let e = Average.recombine([1, 2, 3, 4, 5], &mut rng);
        let f = Average.recombine([100, 200, 300], &mut rng);
        let g = Average.recombine([100, 200, 300, 450], &mut rng);

        assert_eq!(a, Ok([0]));
        assert_eq!(b, Ok([1]));
        assert_eq!(c, Err(AverageError::Wrap));
        assert_eq!(d, Ok([Scored::new(3, 0)]));
        assert_eq!(e, Ok([3]));
        assert_eq!(f, Ok([200]));
        assert_eq!(g, Ok([262]));
    }
}
