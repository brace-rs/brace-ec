use std::convert::Infallible;
use std::ops::Not;

use crate::core::individual::Individual;

use super::Mutator;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Invert<I: Individual>;

impl<I> Mutator<I> for Invert<I>
where
    I: Individual<Genome: Not<Output = I::Genome> + Clone> + From<I::Genome>,
{
    type Error = Infallible;

    fn mutate<Rng>(&self, individual: I, _: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok(individual.genome().clone().not().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::individual::scored::Scored;
    use crate::core::operator::mutator::Mutator;

    use super::Invert;

    #[test]
    fn test_mutate() {
        let mut rng = rand::rng();

        let a = Invert.mutate(true, &mut rng).unwrap();
        let b = Invert.mutate(false, &mut rng).unwrap();
        let c = Invert.mutate(Scored::new(true, 0), &mut rng).unwrap();

        assert!(!a);
        assert!(b);
        assert_eq!(c, Scored::new(false, 0));
    }
}
