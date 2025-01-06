use std::convert::Infallible;
use std::ops::Not;

use crate::core::individual::Individual;

use super::Mutator;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Invert<I: Individual>;

impl<I> Mutator<I> for Invert<I>
where
    I: Individual + Not<Output = I>,
{
    type Error = Infallible;

    fn mutate<Rng>(&self, individual: I, _: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok(individual.not())
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::core::operator::mutator::Mutator;

    use super::Invert;

    #[test]
    fn test_mutate() {
        let mut rng = thread_rng();

        let a = Invert.mutate(true, &mut rng).unwrap();
        let b = Invert.mutate(false, &mut rng).unwrap();

        assert!(!a);
        assert!(b);
    }
}
