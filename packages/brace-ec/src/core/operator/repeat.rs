use rand::Rng;

use crate::core::individual::Individual;
use crate::core::population::Population;

use super::evolver::Evolver;
use super::mutator::Mutator;
use super::recombinator::Recombinator;
use super::selector::Selector;

pub struct Repeat<T> {
    operator: T,
    count: usize,
}

impl<T> Repeat<T> {
    pub fn new(operator: T, count: usize) -> Self {
        Self { operator, count }
    }
}

impl<T> Selector for Repeat<T>
where
    T: Selector<Output: IntoIterator<Item = <T::Population as Population>::Individual>>,
{
    type Population = T::Population;
    type Output = Vec<<T::Population as Population>::Individual>;
    type Error = T::Error;

    fn select<R>(
        &self,
        population: &Self::Population,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>
    where
        R: Rng + ?Sized,
    {
        let mut individuals = Vec::with_capacity(self.count);

        for _ in 0..self.count {
            individuals.extend(self.operator.select(population, rng)?);
        }

        Ok(individuals)
    }
}

impl<I, T> Mutator<I> for Repeat<T>
where
    T: Mutator<I>,
    I: Individual,
{
    type Error = T::Error;

    fn mutate<Rng>(&self, mut individual: I, rng: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        for _ in 0..self.count {
            individual = self.operator.mutate(individual, rng)?;
        }

        Ok(individual)
    }
}

impl<T, P> Recombinator<P> for Repeat<T>
where
    T: Recombinator<P, Output = P>,
    P: Population,
{
    type Output = P;
    type Error = T::Error;

    fn recombine<Rng>(&self, mut parents: P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        for _ in 0..self.count {
            parents = self.operator.recombine(parents, rng)?;
        }

        Ok(parents)
    }
}

impl<T> Evolver for Repeat<T>
where
    T: Evolver,
{
    type Generation = T::Generation;
    type Error = T::Error;

    fn evolve(&self, mut generation: Self::Generation) -> Result<Self::Generation, Self::Error> {
        for _ in 0..self.count {
            generation = self.operator.evolve(generation)?;
        }

        Ok(generation)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::core::individual::Individual;
    use crate::core::operator::evolver::select::Select;
    use crate::core::operator::evolver::Evolver;
    use crate::core::operator::mutator::add::Add;
    use crate::core::operator::mutator::Mutator;
    use crate::core::operator::recombinator::Recombinator;
    use crate::core::operator::selector::first::First;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

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

        let a = population.select(First.repeat(0)).unwrap();
        let b = population.select(First.repeat(1)).unwrap();
        let c = population.select(First.repeat(2)).unwrap();

        assert_eq!(a, []);
        assert_eq!(b, [0]);
        assert_eq!(c, [0, 0]);
    }

    #[test]
    fn test_mutate() {
        let a = 0.mutate(Add(1).repeat(0)).unwrap();
        let b = 1.mutate(Add(1).repeat(2)).unwrap();
        let c = 2.mutate(Add(3).repeat(3)).unwrap();

        assert_eq!(a, 0);
        assert_eq!(b, 3);
        assert_eq!(c, 11);
    }

    #[test]
    fn test_recombine() {
        let population = [0, 1];

        let a = population.recombine(Swap).unwrap();
        let b = population.recombine(Swap.repeat(0)).unwrap();
        let c = population.recombine(Swap.repeat(1)).unwrap();
        let d = population.recombine(Swap.repeat(2)).unwrap();
        let e = population.recombine(Swap.repeat(2).repeat(2)).unwrap();

        assert_eq!(a, [1, 0]);
        assert_eq!(b, [0, 1]);
        assert_eq!(c, [1, 0]);
        assert_eq!(d, [0, 1]);
        assert_eq!(e, [0, 1]);
    }

    #[test]
    fn test_evolve() {
        let a = Select::new(First)
            .repeat(2)
            .evolve((0, [0, 1, 2, 3, 4]))
            .unwrap();

        assert_eq!(a.0, 2);

        let b = Select::new(First)
            .repeat(2)
            .repeat(3)
            .evolve((0, [0, 1, 2, 3, 4]))
            .unwrap();

        assert_eq!(b.0, 6);
    }
}
