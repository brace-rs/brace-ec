use num_traits::{CheckedAdd, One};
use rand::Rng;
use thiserror::Error;

use crate::core::individual::Individual;

use super::Mutator;

#[derive(Clone, Copy, Debug)]
pub struct Add<I: Individual>(pub I);

impl<I> Mutator for Add<I>
where
    I: Individual + CheckedAdd,
{
    type Individual = I;
    type Error = AddError;

    fn mutate<R>(
        &self,
        individual: Self::Individual,
        _: &mut R,
    ) -> Result<Self::Individual, Self::Error>
    where
        R: Rng + ?Sized,
    {
        individual.checked_add(&self.0).ok_or(AddError::Overflow)
    }
}

impl<I> Default for Add<I>
where
    I: Individual + One,
{
    fn default() -> Self {
        Self(I::one())
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

    use crate::core::operator::mutator::add::AddError;
    use crate::core::operator::mutator::Mutator;

    use super::Add;

    #[test]
    fn test_mutate() {
        let mut rng = thread_rng();

        let a = Add(1).mutate(1, &mut rng);
        let b = Add(2).mutate(3, &mut rng);
        let c = Add::default().mutate(5, &mut rng);
        let d = Add(1).mutate(i32::MAX, &mut rng);

        assert_eq!(a, Ok(2));
        assert_eq!(b, Ok(5));
        assert_eq!(c, Ok(6));
        assert_eq!(d, Err(AddError::Overflow));
    }
}
