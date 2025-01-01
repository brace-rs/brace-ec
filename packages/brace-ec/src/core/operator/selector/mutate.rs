use rand::Rng;
use thiserror::Error;

use crate::core::operator::mutator::Mutator;
use crate::core::population::Population;
use crate::util::map::TryMap;

use super::Selector;

pub struct Mutate<S, M> {
    selector: S,
    mutator: M,
}

impl<S, M> Mutate<S, M> {
    pub fn new(selector: S, mutator: M) -> Self {
        Self { selector, mutator }
    }
}

impl<S, M> Selector for Mutate<S, M>
where
    S: Selector<Output: TryMap<Item = <S::Population as Population>::Individual>>,
    M: Mutator<Individual = <S::Population as Population>::Individual>,
{
    type Population = S::Population;
    type Output = S::Output;
    type Error = MutateError<S::Error, M::Error>;

    fn select<R>(
        &self,
        population: &Self::Population,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>
    where
        R: Rng + ?Sized,
    {
        self.selector
            .select(population, rng)
            .map_err(MutateError::Select)?
            .try_map(|individual| self.mutator.mutate(individual, rng))
            .map_err(MutateError::Mutate)
    }
}

#[derive(Debug, Error)]
pub enum MutateError<S, M> {
    #[error(transparent)]
    Select(S),
    #[error(transparent)]
    Mutate(M),
}

#[cfg(test)]
mod tests {
    use crate::core::operator::mutator::add::Add;
    use crate::core::operator::selector::first::First;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    #[test]
    fn test_select() {
        let population = [0];
        let individual = population.select(First.mutate(Add(1))).unwrap()[0];

        assert_eq!(individual, 1);

        let individual = population
            .select(First.mutate(Add(2)).mutate(Add(3)))
            .unwrap()[0];

        assert_eq!(individual, 5);
    }
}
