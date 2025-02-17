use crate::generation::Generation;
use crate::individual::Individual;
use crate::population::Population;

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
    P: Population + ?Sized,
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

impl<G, T, F> Evolver<G> for Inspect<T, F>
where
    G: Generation,
    T: Evolver<G>,
    F: Fn(&G),
{
    type Error = T::Error;

    fn evolve<Rng>(&self, generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operator
            .evolve(generation, rng)
            .inspect(|generation| (self.inspector)(generation))
    }
}

#[cfg(test)]
mod tests {
    use crate::individual::Individual;
    use crate::operator::evolver::select::Select;
    use crate::operator::evolver::Evolver;
    use crate::operator::mutator::add::Add;
    use crate::operator::mutator::Mutator;
    use crate::operator::recombinator::sum::Sum;
    use crate::operator::recombinator::Recombinator;
    use crate::operator::selector::first::First;
    use crate::operator::selector::Selector;
    use crate::population::Population;

    #[test]
    fn test_select() {
        [0, 1, 2, 3, 4]
            .select(First.inspect(|output| assert_eq!(output, &[0])))
            .unwrap();
    }

    #[test]
    fn test_mutate() {
        1.mutated(Add(1).inspect(|individual| assert_eq!(individual, &2)))
            .unwrap();
    }

    #[test]
    fn test_recombine() {
        [2, 2]
            .recombined(Sum.inspect(|output| assert_eq!(output, &[4])))
            .unwrap();
    }

    #[test]
    fn test_evolve() {
        let mut rng = rand::rng();

        Select::fill(First)
            .inspect(|(i, population)| {
                assert_eq!(i, &1);
                assert_eq!(population, &[0, 0, 0, 0, 0]);
            })
            .evolve((0, [0, 1, 2, 3, 4]), &mut rng)
            .unwrap();
    }
}
