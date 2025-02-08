use rayon::iter::ParallelIterator;
use thiserror::Error;

use crate::operator::IntoParallelOperator;
use crate::population::{IterableMutPopulation, ParIterableMutPopulation, ToOwnedPopulation};
use crate::util::iter::{IterableMut, ParIterableMut};

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
    P: ToOwnedPopulation<Owned: IterableMutPopulation> + ?Sized,
    S: Selector<P, Output: IntoIterator<Item = P::Individual>>,
{
    type Output = P::Owned;
    type Error = FillError<S::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut iter = self
            .selector
            .select(population, rng)
            .map_err(FillError::Select)?
            .into_iter();

        let mut selection = population.to_owned();

        selection
            .iter_mut()
            .try_for_each(|individual| match iter.next() {
                Some(item) => {
                    *individual = item;

                    Ok(())
                }
                None => {
                    iter = self
                        .selector
                        .select(population, rng)
                        .map_err(FillError::Select)?
                        .into_iter();

                    match iter.next() {
                        Some(item) => {
                            *individual = item;

                            Ok(())
                        }
                        None => Err(FillError::NotEnough),
                    }
                }
            })?;

        Ok(selection)
    }
}

impl<S> IntoParallelOperator for Fill<S> {
    type Op = ParFill<S>;

    fn parallel(self) -> Self::Op {
        Self::Op {
            selector: self.selector,
        }
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
    P: ToOwnedPopulation<Individual: Send, Owned: ParIterableMutPopulation> + Sync + ?Sized,
    S: Selector<P, Output = [P::Individual; 1], Error: Send> + Sync,
{
    type Output = P::Owned;
    type Error = S::Error;

    fn select<Rng>(&self, population: &P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut selection = population.to_owned();

        selection
            .par_iter_mut()
            .try_for_each_init(rand::rng, |rng, individual| {
                let [item] = self.selector.select(population, rng)?;

                *individual = item;

                Ok(())
            })?;

        Ok(selection)
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
    use crate::operator::selector::best::Best;
    use crate::operator::selector::worst::Worst;
    use crate::operator::selector::Selector;
    use crate::operator::IntoParallelOperator;
    use crate::population::Population;

    #[test]
    fn test_select_fill() {
        let population = [1, 2, 3, 4, 5];

        let a = population.select(Best.fill()).unwrap();
        let b = population.select(Worst.fill()).unwrap();
        let c = population.select(Best.and(Worst).fill()).unwrap();
        let d = population.as_slice().select(Best.fill()).unwrap();

        assert_eq!(a, [5; 5]);
        assert_eq!(b, [1; 5]);
        assert_eq!(c, [5, 1, 5, 1, 5]);
        assert_eq!(d, [5; 5]);
    }

    #[test]
    fn test_select_par_fill() {
        let population = [1, 2, 3, 4, 5];

        let a = population.select(Best.par_fill()).unwrap();
        let b = population.select(Worst.par_fill()).unwrap();
        let c = population
            .select(Best.and(Worst).take::<1>().par_fill())
            .unwrap();
        let d = population.as_slice().select(Best.par_fill()).unwrap();
        let e = population.select(Best.fill().parallel()).unwrap();

        assert_eq!(a, [5; 5]);
        assert_eq!(b, [1; 5]);
        assert_eq!(c, [5; 5]);
        assert_eq!(d, [5; 5]);
        assert_eq!(e, [5; 5]);
    }
}
