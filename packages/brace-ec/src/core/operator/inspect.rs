use crate::core::individual::Individual;
use crate::core::population::Population;

use super::evolver::Evolver;
use super::mutator::Mutator;
use super::recombinator::Recombinator;
use super::selector::Selector;

pub struct Inspect<T, F> {
    operator: T,
    inspector: F,
}

impl<T, F> Inspect<T, F> {
    pub fn new(operator: T, inspector: F) -> Self {
        Self {
            operator,
            inspector,
        }
    }
}

impl<P, T, F> Selector<P> for Inspect<T, F>
where
    P: Population,
    T: Selector<P>,
    F: Fn(&T::Output),
{
    type Output = T::Output;
    type Error = T::Error;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operator
            .select(population, rng)
            .inspect(|output| (self.inspector)(output))
    }
}

impl<I, T, F> Mutator<I> for Inspect<T, F>
where
    I: Individual,
    T: Mutator<I>,
    F: Fn(&I),
{
    type Error = T::Error;

    fn mutate<Rng>(&self, individual: I, rng: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operator
            .mutate(individual, rng)
            .inspect(|individual| (self.inspector)(individual))
    }
}

impl<P, T, F> Recombinator<P> for Inspect<T, F>
where
    P: Population,
    T: Recombinator<P>,
    F: Fn(&T::Output),
{
    type Output = T::Output;
    type Error = T::Error;

    fn recombine<Rng>(&self, parents: P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operator
            .recombine(parents, rng)
            .inspect(|output| (self.inspector)(output))
    }
}

impl<T, F> Evolver for Inspect<T, F>
where
    T: Evolver,
    F: Fn(&T::Generation),
{
    type Generation = T::Generation;
    type Error = T::Error;

    fn evolve(&self, generation: Self::Generation) -> Result<Self::Generation, Self::Error> {
        self.operator
            .evolve(generation)
            .inspect(|generation| (self.inspector)(generation))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::individual::Individual;
    use crate::core::operator::evolver::select::Select;
    use crate::core::operator::evolver::Evolver;
    use crate::core::operator::mutator::add::Add;
    use crate::core::operator::mutator::Mutator;
    use crate::core::operator::recombinator::sum::Sum;
    use crate::core::operator::recombinator::Recombinator;
    use crate::core::operator::selector::first::First;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    #[test]
    fn test_select() {
        [0, 1, 2, 3, 4]
            .select(First.inspect(|output| assert_eq!(output, &[0])))
            .unwrap();
    }

    #[test]
    fn test_mutate() {
        1.mutate(Add(1).inspect(|individual| assert_eq!(individual, &2)))
            .unwrap();
    }

    #[test]
    fn test_recombine() {
        [2, 2]
            .recombine(Sum.inspect(|output| assert_eq!(output, &[4])))
            .unwrap();
    }

    #[test]
    fn test_evolve() {
        Select::new(First)
            .evolve((0, [0, 1, 2, 3, 4]))
            .inspect(|(i, population)| {
                assert_eq!(i, &1);
                assert_eq!(population, &[0, 0, 0, 0, 0]);
            })
            .unwrap();
    }
}
