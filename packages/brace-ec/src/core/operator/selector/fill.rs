use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use thiserror::Error;

use crate::core::population::Population;
use crate::util::map::TryMap;

use super::Selector;

#[derive(Clone, Debug, Default)]
pub struct Fill<S> {
    selector: S,
}

impl<S> Fill<S> {
    pub fn new(selector: S) -> Self {
        Self { selector }
    }
}

impl<P, S> Selector<P> for Fill<S>
where
    P: Population + Clone + TryMap<Item = P::Individual>,
    S: Selector<P, Output: IntoIterator<Item = P::Individual>>,
{
    type Output = P;
    type Error = FillError<S::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut selection = self
            .selector
            .select(population, rng)
            .map_err(FillError::Select)?
            .into_iter();

        let population = population.clone().try_map(|_| match selection.next() {
            Some(individual) => Ok(individual),
            None => {
                selection = self
                    .selector
                    .select(population, rng)
                    .map_err(FillError::Select)?
                    .into_iter();

                match selection.next() {
                    Some(individual) => Ok(individual),
                    None => Err(FillError::NotEnough),
                }
            }
        })?;

        Ok(population)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParFill<S> {
    selector: S,
}

impl<S> ParFill<S> {
    pub fn new(selector: S) -> Self {
        Self { selector }
    }
}

impl<P, S> Selector<P> for ParFill<S>
where
    P: Population<Individual: Send>
        + for<'a> IntoParallelRefMutIterator<'a, Item = &'a mut P::Individual>
        + Clone
        + Sync,
    S: Selector<P, Output = [P::Individual; 1], Error: Send> + Sync,
{
    type Output = P;
    type Error = FillError<S::Error>;

    fn select<Rng>(&self, population: &P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut pop = population.clone();

        pop.par_iter_mut()
            .map_init(rand::thread_rng, |rng, individual| {
                let [out] = self.selector.select(population, rng)?;

                *individual = out;

                Ok(())
            })
            .collect::<Result<(), S::Error>>()
            .map_err(FillError::Select)?;

        Ok(pop)
    }
}

#[derive(Debug, Error)]
pub enum FillError<S> {
    #[error("unable to fill population from selector")]
    NotEnough,
    #[error(transparent)]
    Select(S),
}

#[cfg(test)]
mod tests {
    use crate::core::operator::selector::best::Best;
    use crate::core::operator::selector::worst::Worst;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    #[test]
    fn test_select_fill() {
        let population = [1, 2, 3, 4, 5];

        let a = population.select(Best.fill()).unwrap();
        let b = population.select(Worst.fill()).unwrap();
        let c = population.select(Best.and(Worst).fill()).unwrap();

        assert_eq!(a, [5; 5]);
        assert_eq!(b, [1; 5]);
        assert_eq!(c, [5, 1, 5, 1, 5]);
    }

    #[test]
    fn test_select_par_fill() {
        let population = [1, 2, 3, 4, 5];

        let a = population.select(Best.par_fill()).unwrap();
        let b = population.select(Worst.par_fill()).unwrap();
        let c = population
            .select(Best.and(Worst).take::<1>().par_fill())
            .unwrap();

        assert_eq!(a, [5; 5]);
        assert_eq!(b, [1; 5]);
        assert_eq!(c, [5; 5]);
    }
}
