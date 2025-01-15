use thiserror::Error;

use crate::core::population::Population;
use crate::util::iter::TryFromIterator;

use super::Selector;

pub struct Take<S, const N: usize> {
    selector: S,
}

impl<S, const N: usize> Take<S, N> {
    pub fn new(selector: S) -> Self {
        Self { selector }
    }
}

impl<P, S, const N: usize> Selector<P> for Take<S, N>
where
    P: Population + ?Sized,
    S: Selector<P, Output: IntoIterator<Item = P::Individual>>,
{
    type Output = [P::Individual; N];
    type Error = TakeError<S::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let individuals = self
            .selector
            .select(population, rng)
            .map_err(TakeError::Select)?;

        TryFromIterator::try_from_iter(individuals).map_err(|_| TakeError::NotEnough)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TakeError<S> {
    #[error(transparent)]
    Select(S),
    #[error("not enough individuals")]
    NotEnough,
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::core::operator::recombinator::Recombinator;
    use crate::core::operator::selector::best::Best;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    struct Swap;

    impl Recombinator<[u8; 2]> for Swap {
        type Output = [u8; 2];
        type Error = Infallible;

        fn recombine<Rng>(&self, parents: [u8; 2], _: &mut Rng) -> Result<Self::Output, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            Ok([parents[1], parents[0]])
        }
    }

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];

        let a = population.select(Best.repeat(5).take::<2>()).unwrap();
        let b = population
            .select(Best.repeat(3).take::<2>().recombine(Swap))
            .unwrap();
        let c = population.select(Best.take::<2>());

        assert_eq!(a, [4, 4]);
        assert_eq!(b, [4, 4]);
        assert!(c.is_err());
    }
}
