use thiserror::Error;

use crate::core::operator::recombinator::Recombinator;
use crate::core::population::Population;

use super::Selector;

pub struct Recombine<S, R> {
    selector: S,
    recombinator: R,
}

impl<S, R> Recombine<S, R> {
    pub fn new(selector: S, recombinator: R) -> Self {
        Self {
            selector,
            recombinator,
        }
    }
}

impl<P, S, R> Selector<P> for Recombine<S, R>
where
    P: Population + ?Sized,
    S: Selector<P>,
    R: Recombinator<S::Output>,
{
    type Output = R::Output;
    type Error = RecombineError<S::Error, R::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let parents = self
            .selector
            .select(population, rng)
            .map_err(RecombineError::Select)?;

        self.recombinator
            .recombine(parents, rng)
            .map_err(RecombineError::Recombine)
    }
}

#[derive(Debug, Error)]
pub enum RecombineError<S, R> {
    #[error(transparent)]
    Select(S),
    #[error(transparent)]
    Recombine(R),
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::core::operator::recombinator::sum::Sum;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    struct LastTwo;

    impl Selector<[u8; 5]> for LastTwo {
        type Output = [u8; 2];
        type Error = Infallible;

        fn select<Rng>(
            &self,
            population: &[u8; 5],
            _: &mut Rng,
        ) -> Result<Self::Output, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            Ok([population[3], population[4]])
        }
    }

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];
        let individual = population.select(LastTwo.recombine(Sum)).unwrap()[0];

        assert_eq!(individual, 7);
    }
}
