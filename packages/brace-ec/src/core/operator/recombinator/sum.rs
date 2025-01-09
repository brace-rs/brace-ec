use thiserror::Error;

use crate::core::individual::Individual;
use crate::core::population::IterablePopulation;
use crate::util::sum::CheckedSum;

use super::Recombinator;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Sum<P: IterablePopulation>;

impl<P, G> Recombinator<P> for Sum<P>
where
    P: IterablePopulation<Individual: Individual<Genome = G> + From<G>>,
    G: for<'a> CheckedSum<&'a G>,
{
    type Output = [P::Individual; 1];
    type Error = SumError;

    fn recombine<Rng>(&self, parents: P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok([G::checked_sum(parents.iter().map(Individual::genome))
            .ok_or(SumError::Overflow)?
            .into()])
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SumError {
    #[error("summation would overflow")]
    Overflow,
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::core::individual::scored::Scored;
    use crate::core::operator::recombinator::Recombinator;

    use super::{Sum, SumError};

    #[test]
    fn test_recombine() {
        let mut rng = thread_rng();

        let a = Sum.recombine([0, 0], &mut rng);
        let b = Sum.recombine([1, 2], &mut rng);
        let c = Sum.recombine([1, i32::MAX], &mut rng);
        let d = Sum.recombine([Scored::new(3, 0), Scored::new(4, 0)], &mut rng);

        assert_eq!(a, Ok([0]));
        assert_eq!(b, Ok([3]));
        assert_eq!(c, Err(SumError::Overflow));
        assert_eq!(d, Ok([Scored::new(7, 0)]));
    }
}
