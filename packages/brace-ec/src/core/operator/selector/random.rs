use rand::seq::IteratorRandom;
use rand::Rng;
use thiserror::Error;

use crate::core::population::IterablePopulation;

use super::Selector;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Random<P: IterablePopulation>;

impl<P> Selector for Random<P>
where
    P: IterablePopulation<Individual: Clone>,
{
    type Population = P;
    type Output = [P::Individual; 1];
    type Error = RandomError;

    fn select<R: Rng>(&self, population: &P, rng: &mut R) -> Result<Self::Output, Self::Error> {
        Ok([population
            .iter()
            .choose(rng)
            .ok_or(RandomError::Empty)?
            .clone()])
    }
}

#[derive(Debug, Error)]
pub enum RandomError {
    #[error("Empty population")]
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::core::population::Population;

    use super::Random;

    #[test]
    fn test_select() {
        let population = [[0], [1], [2], [3], [4]];

        for _ in 0..10 {
            let individual = population.select(Random).unwrap()[0];

            assert!(population.contains(&individual));
        }
    }
}
