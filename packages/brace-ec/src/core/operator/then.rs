use rand::Rng;
use thiserror::Error;

use crate::core::individual::Individual;
use crate::core::population::Population;

use super::evolver::Evolver;
use super::mutator::Mutator;
use super::recombinator::Recombinator;
use super::selector::Selector;

pub struct Then<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> Then<L, R> {
    pub fn new(lhs: L, rhs: R) -> Self {
        Self { lhs, rhs }
    }
}

impl<L, R> Selector for Then<L, R>
where
    L: Selector,
    R: Selector<Population = L::Output>,
{
    type Population = L::Population;
    type Output = R::Output;
    type Error = ThenError<L::Error, R::Error>;

    fn select<G>(
        &self,
        population: &Self::Population,
        rng: &mut G,
    ) -> Result<Self::Output, Self::Error>
    where
        G: Rng + ?Sized,
    {
        let population = self.lhs.select(population, rng).map_err(ThenError::Left)?;

        self.rhs.select(&population, rng).map_err(ThenError::Right)
    }
}

impl<T, L, R> Mutator<T> for Then<L, R>
where
    T: Individual,
    L: Mutator<T>,
    R: Mutator<T>,
{
    type Error = ThenError<L::Error, R::Error>;

    fn mutate<Rng>(&self, individual: T, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.rhs
            .mutate(
                self.lhs.mutate(individual, rng).map_err(ThenError::Left)?,
                rng,
            )
            .map_err(ThenError::Right)
    }
}

impl<P, L, R> Recombinator<P> for Then<L, R>
where
    P: Population,
    L: Recombinator<P>,
    R: Recombinator<L::Output>,
{
    type Output = R::Output;
    type Error = ThenError<L::Error, R::Error>;

    fn recombine<Rng>(&self, parents: P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        // Note: Using this form because rust-analyzer gets confused.
        Recombinator::recombine(
            &self.rhs,
            Recombinator::recombine(&self.lhs, parents, rng).map_err(ThenError::Left)?,
            rng,
        )
        .map_err(ThenError::Right)
    }
}

impl<L, R> Evolver for Then<L, R>
where
    L: Evolver,
    R: Evolver<Generation = L::Generation>,
{
    type Generation = L::Generation;
    type Error = ThenError<L::Error, R::Error>;

    fn evolve(&self, generation: Self::Generation) -> Result<Self::Generation, Self::Error> {
        self.rhs
            .evolve(self.lhs.evolve(generation).map_err(ThenError::Left)?)
            .map_err(ThenError::Right)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ThenError<L, R> {
    #[error(transparent)]
    Left(L),
    #[error(transparent)]
    Right(R),
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use rand::Rng;

    use crate::core::individual::Individual;
    use crate::core::operator::evolver::select::Select;
    use crate::core::operator::evolver::Evolver;
    use crate::core::operator::mutator::add::Add;
    use crate::core::operator::mutator::Mutator;
    use crate::core::operator::recombinator::Recombinator;
    use crate::core::operator::selector::best::Best;
    use crate::core::operator::selector::first::First;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    struct All;

    impl Selector for All {
        type Population = [i32; 5];
        type Output = [i32; 5];
        type Error = Infallible;

        fn select<R>(
            &self,
            population: &Self::Population,
            _: &mut R,
        ) -> Result<Self::Output, Self::Error>
        where
            R: Rng + ?Sized,
        {
            Ok(*population)
        }
    }

    struct Swap;

    impl Recombinator<[u8; 2]> for Swap {
        type Output = [u8; 2];
        type Error = Infallible;

        fn recombine<Rng>(&self, parents: [u8; 2], _: &mut Rng) -> Result<Self::Output, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            Ok([parents[1], parents[0]])
        }
    }

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];

        let a = population.select(All.then(Best)).unwrap();
        let b = population.select(All.then(First)).unwrap();

        assert_eq!(a, [4]);
        assert_eq!(b, [0]);
    }

    #[test]
    fn test_mutate() {
        let a = 0.mutate(Add(1).then(Add(2))).unwrap();
        let b = 1.mutate(Add(2).then(Add(1))).unwrap();

        assert_eq!(a, 3);
        assert_eq!(b, 4);
    }

    #[test]
    fn test_recombine() {
        let population = [0, 1];

        let a = population.recombine(Swap).unwrap();
        let b = population.recombine(Swap.then(Swap)).unwrap();
        let c = population.recombine(Swap.then(Swap).then(Swap)).unwrap();

        assert_eq!(a, [1, 0]);
        assert_eq!(b, [0, 1]);
        assert_eq!(c, [1, 0]);
    }

    #[test]
    fn test_evolve() {
        let a = Select::new(Best)
            .then(Select::new(First))
            .evolve((0, [0, 1, 2, 3, 4]))
            .unwrap();

        let b = Select::new(First)
            .then(Select::new(Best))
            .evolve((0, [0, 1, 2, 3, 4]))
            .unwrap();

        assert_eq!(a, (2, [4, 4, 4, 4, 4]));
        assert_eq!(b, (2, [0, 0, 0, 0, 0]));
    }
}
