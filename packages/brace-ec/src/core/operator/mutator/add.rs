use num_traits::{CheckedAdd, One};
use thiserror::Error;

use crate::core::individual::Individual;

use super::Mutator;

#[derive(Clone, Copy, Debug)]
pub struct Add<I: Individual>(pub I::Genome);

impl<I> Mutator<I> for Add<I>
where
    I: Individual<Genome: CheckedAdd>,
{
    type Error = AddError;

    fn mutate<Rng>(&self, mut individual: I, _: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let genome = individual
            .genome()
            .checked_add(&self.0)
            .ok_or(AddError::Overflow)?;

        *individual.genome_mut() = genome;

        Ok(individual)
    }
}

impl<I> Default for Add<I>
where
    I: Individual<Genome: One>,
{
    fn default() -> Self {
        Self(I::Genome::one())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AddError {
    #[error("addition would overflow")]
    Overflow,
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::core::individual::scored::Scored;
    use crate::core::operator::mutator::Mutator;

    use super::{Add, AddError};

    #[test]
    fn test_mutate() {
        let mut rng = thread_rng();

        let a = Add(1).mutate(1, &mut rng);
        let b = Add(2).mutate(3, &mut rng);
        let c = Add::default().mutate(5, &mut rng);
        let d = Add(1).mutate(i32::MAX, &mut rng);
        let e = Add(5).mutate(Scored::new(10, 0), &mut rng);

        assert_eq!(a, Ok(2));
        assert_eq!(b, Ok(5));
        assert_eq!(c, Ok(6));
        assert_eq!(d, Err(AddError::Overflow));
        assert_eq!(e, Ok(Scored::new(15, 0)));
    }
}
