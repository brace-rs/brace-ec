use thiserror::Error;

use crate::core::population::Population;
use crate::util::sum::CheckedSum;

use super::Recombinator;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Sum<P: Population>;

impl<P> Recombinator<P> for Sum<P>
where
    P: Population + CheckedSum<P::Individual>,
{
    type Output = [P::Individual; 1];
    type Error = SumError;

    fn recombine<Rng>(&self, parents: P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok([parents.checked_sum().ok_or(SumError::Overflow)?])
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

    use crate::core::operator::recombinator::Recombinator;

    use super::{Sum, SumError};

    #[test]
    fn test_recombine() {
        let mut rng = thread_rng();

        let a = Sum.recombine([0, 0], &mut rng);
        let b = Sum.recombine([1, 2], &mut rng);
        let c = Sum.recombine([1, i32::MAX], &mut rng);

        assert_eq!(a, Ok([0]));
        assert_eq!(b, Ok([3]));
        assert_eq!(c, Err(SumError::Overflow));
    }
}
