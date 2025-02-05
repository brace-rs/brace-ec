use crate::generation::Generation;
use crate::individual::Individual;
use crate::population::Population;

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

impl<P, T> Selector<P> for Repeat<T>
where
    P: Population + ?Sized,
    T: Selector<P, Output: IntoIterator<Item = P::Individual>>,
{
    type Output = Vec<P::Individual>;
    type Error = T::Error;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
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

impl<G, T> Evolver<G> for Repeat<T>
where
    G: Generation,
    T: Evolver<G>,
{
    type Error = T::Error;

    fn evolve<Rng>(&self, mut generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        for _ in 0..self.count {
            generation = self.operator.evolve(generation, rng)?;
        }

        Ok(generation)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::individual::Individual;
    use crate::operator::evolver::select::Select;
    use crate::operator::evolver::Evolver;
    use crate::operator::mutator::add::Add;
    use crate::operator::mutator::Mutator;
    use crate::operator::recombinator::Recombinator;
    use crate::operator::selector::first::First;
    use crate::operator::selector::Selector;
    use crate::population::Population;

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
        let a = 0.mutated(Add(1).repeat(0)).unwrap();
        let b = 1.mutated(Add(1).repeat(2)).unwrap();
        let c = 2.mutated(Add(3).repeat(3)).unwrap();

        assert_eq!(a, 0);
        assert_eq!(b, 3);
        assert_eq!(c, 11);
    }

    #[test]
    fn test_recombine() {
        let population = [0, 1];

        let a = population.recombined(Swap).unwrap();
        let b = population.recombined(Swap.repeat(0)).unwrap();
        let c = population.recombined(Swap.repeat(1)).unwrap();
        let d = population.recombined(Swap.repeat(2)).unwrap();
        let e = population.recombined(Swap.repeat(2).repeat(2)).unwrap();

        assert_eq!(a, [1, 0]);
        assert_eq!(b, [0, 1]);
        assert_eq!(c, [1, 0]);
        assert_eq!(d, [0, 1]);
        assert_eq!(e, [0, 1]);
    }

    #[test]
    fn test_evolve() {
        let mut rng = rand::rng();

        let a = Select::fill(First)
            .repeat(2)
            .evolve((0, [0, 1, 2, 3, 4]), &mut rng)
            .unwrap();

        assert_eq!(a.0, 2);

        let b = Select::fill(First)
            .repeat(2)
            .repeat(3)
            .evolve((0, [0, 1, 2, 3, 4]), &mut rng)
            .unwrap();

        assert_eq!(b.0, 6);
    }
}
