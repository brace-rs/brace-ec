use thiserror::Error;

use crate::core::fitness::Fitness;
use crate::core::population::{IterablePopulation, Population};

use super::Selector;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Worst<P: Population>;

impl<P> Selector<P> for Worst<P>
where
    P: IterablePopulation<Individual: Clone + Fitness>,
{
    type Output = [P::Individual; 1];
    type Error = WorstError;

    fn select<Rng>(&self, population: &P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok([population
            .iter()
            .min_by_key(|individual| individual.fitness())
            .ok_or(WorstError::Empty)?
            .clone()])
    }
}

#[derive(Debug, Error)]
pub enum WorstError {
    #[error("empty population")]
    Empty,
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use crate::core::individual::scored::Scored;
    use crate::core::population::Population;

    use super::Worst;

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];
        let individual = population.select(Worst).unwrap()[0];

        assert_eq!(individual, 0);

        let population = [Reverse(0), Reverse(1), Reverse(2), Reverse(3), Reverse(4)];
        let individual = population.select(Worst).unwrap()[0];

        assert_eq!(individual, Reverse(4));

        let population = [Scored::new(0, 0), Scored::new(10, 10), Scored::new(20, 5)];
        let individual = population.select(Worst).unwrap()[0];

        assert_eq!(individual, Scored::new(0, 0));

        let population = [
            Reverse(Scored::new(30, 3)),
            Reverse(Scored::new(10, 10)),
            Reverse(Scored::new(20, 5)),
        ];
        let individual = population.select(Worst).unwrap()[0];

        assert_eq!(individual, Reverse(Scored::new(10, 10)));
    }
}
