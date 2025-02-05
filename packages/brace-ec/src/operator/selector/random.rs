use rand::seq::IteratorRandom;
use thiserror::Error;

use crate::population::IterablePopulation;

use super::Selector;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Random<P: IterablePopulation + ?Sized>;

impl<P> Selector<P> for Random<P>
where
    P: IterablePopulation<Individual: Clone> + ?Sized,
{
    type Output = [P::Individual; 1];
    type Error = RandomError;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
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
    use crate::population::Population;

    use super::Random;

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];

        for _ in 0..10 {
            let individual = population.select(Random).unwrap()[0];

            assert!(population.contains(&individual));
        }
    }
}
