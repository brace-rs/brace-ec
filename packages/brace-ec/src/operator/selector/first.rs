use thiserror::Error;

use crate::population::IterablePopulation;

use super::Selector;

#[ghost::phantom]
#[derive(Clone, Copy, Debug)]
pub struct First<P: IterablePopulation + ?Sized>;

impl<P> Selector<P> for First<P>
where
    P: IterablePopulation<Individual: Clone> + ?Sized,
{
    type Output = [P::Individual; 1];
    type Error = FirstError;

    fn select<Rng>(&self, population: &P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok([population.iter().next().ok_or(FirstError::Empty)?.clone()])
    }
}

#[derive(Debug, Error)]
pub enum FirstError {
    #[error("Empty population")]
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::population::Population;

    use super::First;

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];
        let individual = population.select(First).unwrap()[0];

        assert_eq!(population[0], individual);
    }
}
