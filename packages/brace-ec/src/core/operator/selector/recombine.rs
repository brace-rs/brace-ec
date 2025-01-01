use rand::Rng;
use thiserror::Error;

use crate::core::operator::recombinator::Recombinator;

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

impl<S, R> Selector for Recombine<S, R>
where
    S: Selector,
    R: Recombinator<Parents = S::Output>,
{
    type Population = S::Population;
    type Output = R::Output;
    type Error = RecombineError<S::Error, R::Error>;

    fn select<G>(
        &self,
        population: &Self::Population,
        rng: &mut G,
    ) -> Result<Self::Output, Self::Error>
    where
        G: Rng + ?Sized,
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

    use rand::Rng;

    use crate::core::operator::recombinator::sum::Sum;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    struct LastTwo;

    impl Selector for LastTwo {
        type Population = [u8; 5];
        type Output = [u8; 2];
        type Error = Infallible;

        fn select<R>(
            &self,
            population: &Self::Population,
            _: &mut R,
        ) -> Result<Self::Output, Self::Error>
        where
            R: Rng + ?Sized,
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
