use thiserror::Error;

use crate::core::individual::Individual;
use crate::core::population::{IterablePopulation, Population};

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
    use crate::core::individual::reversed::Reversed;
    use crate::core::individual::scored::Scored;
    use crate::core::individual::Individual;
    use crate::core::population::Population;

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

        let population = [Scored::new(0, 0), Scored::new(10, 10), Scored::new(20, 5)];
        let individual = population.select(Best).unwrap()[0];

        assert_eq!(individual, Scored::new(10, 10));

        let population = [
            Scored::new(30, 3).reversed(),
            Scored::new(10, 10).reversed(),
            Scored::new(20, 5).reversed(),
        ];
        let individual = population.select(Best).unwrap()[0];

        assert_eq!(individual, Reversed::new(Scored::new(30, 3)));
    }
}
