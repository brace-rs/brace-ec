use thiserror::Error;

use crate::population::Population;

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

impl<P, L, R> Selector<P> for And<L, R>
where
    P: Population + ?Sized,
    L: Selector<P, Output: IntoIterator<Item = P::Individual>>,
    R: Selector<P, Output: IntoIterator<Item = P::Individual>>,
{
    type Output = Vec<P::Individual>;
    type Error = AndError<L::Error, R::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
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
    use crate::operator::selector::best::Best;
    use crate::operator::selector::first::First;
    use crate::operator::selector::Selector;
    use crate::population::Population;

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];

        let a = population.select(First.and(Best)).unwrap();
        let b = population.select(Best.and(First)).unwrap();

        assert_eq!(a, [0, 4]);
        assert_eq!(b, [4, 0]);
    }
}
