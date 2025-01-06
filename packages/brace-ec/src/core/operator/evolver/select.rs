use std::marker::PhantomData;

use rand::thread_rng;
use thiserror::Error;

use crate::core::generation::Generation;
use crate::core::operator::selector::Selector;
use crate::core::population::Population;
use crate::util::map::TryMap;

use super::Evolver;

#[derive(Clone, Debug, Default)]
pub struct Select<P, S> {
    selector: S,
    marker: PhantomData<fn() -> P>,
}

impl<P, S> Select<P, S> {
    pub fn new(selector: S) -> Self {
        Self {
            selector,
            marker: PhantomData,
        }
    }
}

impl<P, S> Evolver for Select<P, S>
where
    P: Population + Clone + TryMap<Item = P::Individual>,
    S: Selector<P, Output: IntoIterator<Item = P::Individual>>,
{
    type Generation = (u64, P);
    type Error = SelectError<S::Error>;

    fn evolve(&self, mut generation: Self::Generation) -> Result<Self::Generation, Self::Error> {
        let mut rng = thread_rng();
        let mut selection = self
            .selector
            .select(generation.population(), &mut rng)
            .map_err(SelectError::Select)?
            .into_iter();

        let population = generation
            .population()
            .clone()
            .try_map(|_| match selection.next() {
                Some(individual) => Ok(individual),
                None => {
                    selection = self
                        .selector
                        .select(generation.population(), &mut rng)
                        .map_err(SelectError::Select)?
                        .into_iter();

                    match selection.next() {
                        Some(individual) => Ok(individual),
                        None => Err(SelectError::NotEnough),
                    }
                }
            })?;

        generation.0 += 1;
        generation.1 = population;

        Ok(generation)
    }
}

#[derive(Debug, Error)]
pub enum SelectError<S> {
    #[error("unable to fill population from selector")]
    NotEnough,
    #[error(transparent)]
    Select(S),
}

#[cfg(test)]
mod tests {
    use crate::core::operator::evolver::Evolver;
    use crate::core::operator::selector::random::Random;

    use super::Select;

    #[test]
    fn test_evolve() {
        let evolver = Select::new(Random);
        let population = [0, 1, 2, 3, 4];
        let generation = evolver.evolve((0, population)).unwrap();

        assert_eq!(generation.0, 1);
        assert!(generation.1.iter().all(|i| population.contains(i)));

        let generation = evolver.evolve(generation).unwrap();

        assert_eq!(generation.0, 2);
        assert!(generation.1.iter().all(|i| population.contains(i)));
    }
}
