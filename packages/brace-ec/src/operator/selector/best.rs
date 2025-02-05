use thiserror::Error;

use crate::individual::Individual;
use crate::population::{IterablePopulation, Population};

use super::Selector;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct Best<P: Population + ?Sized>;

impl<P> Selector<P> for Best<P>
where
    P: IterablePopulation<Individual: Clone> + ?Sized,
{
    type Output = [P::Individual; 1];
    type Error = BestError;

    fn select<Rng>(&self, population: &P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok([population
            .iter()
            .max_by_key(|individual| individual.fitness())
            .ok_or(BestError::Empty)?
            .clone()])
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BestError {
    #[error("empty population")]
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::individual::evaluated::Evaluated;
    use crate::individual::reversed::Reversed;
    use crate::individual::Individual;
    use crate::population::Population;

    use super::Best;

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];
        let individual = population.select(Best).unwrap()[0];

        assert_eq!(individual, 4);

        let population = [
            Reversed::new(0),
            Reversed::new(1),
            Reversed::new(2),
            Reversed::new(3),
            Reversed::new(4),
        ];
        let individual = population.select(Best).unwrap()[0];

        assert_eq!(individual, Reversed::new(0));

        let population = [
            Evaluated::new(0, 0),
            Evaluated::new(10, 10),
            Evaluated::new(20, 5),
        ];
        let individual = population.select(Best).unwrap()[0];

        assert_eq!(individual, Evaluated::new(10, 10));

        let population = [
            Evaluated::new(30, 3).reversed(),
            Evaluated::new(10, 10).reversed(),
            Evaluated::new(20, 5).reversed(),
        ];
        let individual = population.select(Best).unwrap()[0];

        assert_eq!(individual, Reversed::new(Evaluated::new(30, 3)));
    }
}
