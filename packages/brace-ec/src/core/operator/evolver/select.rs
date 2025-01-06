use std::marker::PhantomData;

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

impl<P, S> Evolver<(u64, P)> for Select<P, S>
where
    P: Population + Clone + TryMap<Item = P::Individual>,
    S: Selector<P, Output: IntoIterator<Item = P::Individual>>,
{
    type Error = SelectError<S::Error>;

    fn evolve<Rng>(&self, mut generation: (u64, P), rng: &mut Rng) -> Result<(u64, P), Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut selection = self
            .selector
            .select(generation.population(), rng)
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
                        .select(generation.population(), rng)
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
        let mut rng = rand::thread_rng();

        let evolver = Select::new(Random);
        let population = [0, 1, 2, 3, 4];
        let generation = evolver.evolve((0, population), &mut rng).unwrap();

        assert_eq!(generation.0, 1);
        assert!(generation.1.iter().all(|i| population.contains(i)));

        let generation = evolver.evolve(generation, &mut rng).unwrap();

        assert_eq!(generation.0, 2);
        assert!(generation.1.iter().all(|i| population.contains(i)));
    }
}
