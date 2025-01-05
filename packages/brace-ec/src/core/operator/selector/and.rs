use rand::Rng;
use thiserror::Error;

use crate::core::population::Population;

use super::Selector;

pub struct And<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> And<L, R> {
    pub fn new(lhs: L, rhs: R) -> Self {
        Self { lhs, rhs }
    }
}

impl<L, R> Selector for And<L, R>
where
    L: Selector<Output: IntoIterator<Item = <L::Population as Population>::Individual>>,
    R: Selector<
        Population = L::Population,
        Output: IntoIterator<Item = <L::Population as Population>::Individual>,
    >,
{
    type Population = L::Population;
    type Output = Vec<<L::Population as Population>::Individual>;
    type Error = AndError<L::Error, R::Error>;

    fn select<G>(
        &self,
        population: &Self::Population,
        rng: &mut G,
    ) -> Result<Self::Output, Self::Error>
    where
        G: Rng + ?Sized,
    {
        Ok(self
            .lhs
            .select(population, rng)
            .map_err(AndError::Left)?
            .into_iter()
            .chain(self.rhs.select(population, rng).map_err(AndError::Right)?)
            .collect())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AndError<L, R> {
    #[error(transparent)]
    Left(L),
    #[error(transparent)]
    Right(R),
}

#[cfg(test)]
mod tests {
    use crate::core::operator::selector::best::Best;
    use crate::core::operator::selector::first::First;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];

        let a = population.select(First.and(Best)).unwrap();
        let b = population.select(Best.and(First)).unwrap();

        assert_eq!(a, [0, 4]);
        assert_eq!(b, [4, 0]);
    }
}
