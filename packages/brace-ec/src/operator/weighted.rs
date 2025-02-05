use std::error::Error;

use rand::seq::IndexedRandom;

use crate::generation::Generation;
use crate::individual::Individual;
use crate::population::Population;

use super::evolver::{DynEvolver, Evolver};
use super::generator::{DynGenerator, Generator};
use super::mutator::{DynMutator, Mutator};
use super::recombinator::{DynRecombinator, Recombinator};
use super::scorer::{DynScorer, Scorer};
use super::selector::{DynSelector, Selector};

pub struct Weighted<T>
where
    T: ?Sized,
{
    operators: Vec<(Box<T>, u64)>,
}

impl<P, O> Weighted<dyn DynSelector<P, O>>
where
    P: Population + ?Sized,
    O: Population<Individual = P::Individual>,
{
    pub fn selector<S>(selector: S, weight: u64) -> Self
    where
        S: Selector<P, Output = O, Error: Error + 'static> + 'static,
    {
        Self {
            operators: vec![(Box::new(selector), weight)],
        }
    }

    pub fn with_selector<S>(mut self, selector: S, weight: u64) -> Self
    where
        S: Selector<P, Output: Into<O>, Error: Error + 'static> + 'static,
    {
        self.operators.push((Box::new(selector), weight));
        self
    }
}

impl<P, O, E> Selector<P> for Weighted<dyn DynSelector<P, O, E>>
where
    P: Population,
    O: Population<Individual = P::Individual>,
{
    type Output = O;
    type Error = E;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operators
            .choose_weighted(rng, |(_, weight)| *weight)
            .expect("cannot construct without at least 1 operator")
            .0
            .select(population, rng)
    }
}

impl<I> Weighted<dyn DynMutator<I>>
where
    I: Individual,
{
    pub fn mutator<M>(mutator: M, weight: u64) -> Self
    where
        M: Mutator<I, Error: Error + 'static> + 'static,
    {
        Self {
            operators: vec![(Box::new(mutator), weight)],
        }
    }

    pub fn with_mutator<M>(mut self, mutator: M, weight: u64) -> Self
    where
        M: Mutator<I, Error: Error + 'static> + 'static,
    {
        self.operators.push((Box::new(mutator), weight));
        self
    }
}

impl<I, E> Mutator<I> for Weighted<dyn DynMutator<I, E>>
where
    I: Individual,
{
    type Error = E;

    fn mutate<Rng>(&self, individual: I, rng: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operators
            .choose_weighted(rng, |(_, weight)| *weight)
            .expect("cannot construct without at least 1 operator")
            .0
            .mutate(individual, rng)
    }
}

impl<P, O> Weighted<dyn DynRecombinator<P, O>>
where
    P: Population,
    O: Population<Individual = P::Individual>,
{
    pub fn recombinator<R>(recombinator: R, weight: u64) -> Self
    where
        R: Recombinator<P, Output = O, Error: Error + 'static> + 'static,
    {
        Self {
            operators: vec![(Box::new(recombinator), weight)],
        }
    }

    pub fn with_recombinator<R>(mut self, recombinator: R, weight: u64) -> Self
    where
        R: Recombinator<P, Output: Into<O>, Error: Error + 'static> + 'static,
    {
        self.operators.push((Box::new(recombinator), weight));
        self
    }
}

impl<P, O, E> Recombinator<P> for Weighted<dyn DynRecombinator<P, O, E>>
where
    P: Population,
    O: Population<Individual = P::Individual>,
{
    type Output = O;
    type Error = E;

    fn recombine<Rng>(&self, population: P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operators
            .choose_weighted(rng, |(_, weight)| *weight)
            .expect("cannot construct without at least 1 operator")
            .0
            .recombine(population, rng)
    }
}

impl<G> Weighted<dyn DynEvolver<G>>
where
    G: Generation,
{
    pub fn evolver<E>(evolver: E, weight: u64) -> Self
    where
        E: Evolver<G, Error: Error + 'static> + 'static,
    {
        Self {
            operators: vec![(Box::new(evolver), weight)],
        }
    }

    pub fn with_evolver<M>(mut self, evolver: M, weight: u64) -> Self
    where
        M: Evolver<G, Error: Error + 'static> + 'static,
    {
        self.operators.push((Box::new(evolver), weight));
        self
    }
}

impl<G, E> Evolver<G> for Weighted<dyn DynEvolver<G, E>>
where
    G: Generation,
{
    type Error = E;

    fn evolve<Rng>(&self, generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operators
            .choose_weighted(rng, |(_, weight)| *weight)
            .expect("cannot construct without at least 1 operator")
            .0
            .evolve(generation, rng)
    }
}

impl<I> Weighted<dyn DynScorer<I>>
where
    I: Individual,
{
    pub fn scorer<S>(scorer: S, weight: u64) -> Self
    where
        S: Scorer<I, Error: Error + 'static> + 'static,
    {
        Self {
            operators: vec![(Box::new(scorer), weight)],
        }
    }

    pub fn with_scorer<S>(mut self, scorer: S, weight: u64) -> Self
    where
        S: Scorer<I, Error: Error + 'static> + 'static,
    {
        self.operators.push((Box::new(scorer), weight));
        self
    }
}

impl<I, E> Scorer<I> for Weighted<dyn DynScorer<I, E>>
where
    I: Individual,
{
    type Error = E;

    fn score<Rng>(&self, individual: &I, rng: &mut Rng) -> Result<I::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operators
            .choose_weighted(rng, |(_, weight)| *weight)
            .expect("cannot construct without at least 1 operator")
            .0
            .score(individual, rng)
    }
}

impl<T> Weighted<dyn DynGenerator<T>> {
    pub fn generator<G>(generator: G, weight: u64) -> Self
    where
        G: Generator<T, Error: Error + 'static> + 'static,
    {
        Self {
            operators: vec![(Box::new(generator), weight)],
        }
    }

    pub fn with_generator<G>(mut self, generator: G, weight: u64) -> Self
    where
        G: Generator<T, Error: Error + 'static> + 'static,
    {
        self.operators.push((Box::new(generator), weight));
        self
    }
}

impl<T, E> Generator<T> for Weighted<dyn DynGenerator<T, E>> {
    type Error = E;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operators
            .choose_weighted(rng, |(_, weight)| *weight)
            .expect("cannot construct without at least 1 operator")
            .0
            .generate(rng)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::operator::evolver::select::Select;
    use crate::operator::evolver::Evolver;
    use crate::operator::generator::random::Random;
    use crate::operator::generator::Generator;
    use crate::operator::mutator::add::Add;
    use crate::operator::mutator::Mutator;
    use crate::operator::recombinator::sum::Sum;
    use crate::operator::recombinator::Recombinator;
    use crate::operator::scorer::function::Function;
    use crate::operator::scorer::Scorer;
    use crate::operator::selector::best::Best;
    use crate::operator::selector::worst::Worst;
    use crate::operator::selector::Selector;

    use super::Weighted;

    #[test]
    fn test_select() {
        let mut rng = rand::rng();

        for _ in 0..10 {
            let a = Weighted::selector(Best, 1)
                .with_selector(Worst, 1)
                .select(&[0, 1, 2, 3, 4], &mut rng)
                .unwrap()[0];
            let b = Weighted::selector(Best, 1)
                .with_selector(Worst, 0)
                .select(&[0, 1, 2, 3, 4], &mut rng)
                .unwrap()[0];
            let c = Weighted::selector(Best, 0)
                .with_selector(Worst, 1)
                .select(&[0, 1, 2, 3, 4], &mut rng)
                .unwrap()[0];

            assert!(a == 0 || a == 4);
            assert_eq!(b, 4);
            assert_eq!(c, 0);
        }
    }

    #[test]
    fn test_mutate() {
        let mut rng = rand::rng();

        for _ in 0..10 {
            let a = Weighted::mutator(Add(1), 1)
                .with_mutator(Add(2), 1)
                .mutate(5, &mut rng)
                .unwrap();
            let b = Weighted::mutator(Add(1), 1)
                .with_mutator(Add(2), 0)
                .mutate(5, &mut rng)
                .unwrap();
            let c = Weighted::mutator(Add(1), 0)
                .with_mutator(Add(2), 1)
                .mutate(5, &mut rng)
                .unwrap();

            assert!(a == 6 || a == 7);
            assert_eq!(b, 6);
            assert_eq!(c, 7);
        }
    }

    #[test]
    fn test_recombine() {
        let mut rng = rand::rng();

        let a = Weighted::recombinator(Sum, 1)
            .with_recombinator(Sum, 1)
            .recombine([1, 2], &mut rng)
            .unwrap()[0];

        assert_eq!(a, 3);
    }

    #[test]
    fn test_evolve() {
        let mut rng = rand::rng();

        for _ in 0..10 {
            let a = Weighted::evolver(Select::fill(Best), 1)
                .with_evolver(Select::fill(Worst), 1)
                .evolve((0, [0, 1, 2, 3, 4]), &mut rng)
                .unwrap();
            let b = Weighted::evolver(Select::fill(Best), 1)
                .with_evolver(Select::fill(Worst), 0)
                .evolve((0, [0, 1, 2, 3, 4]), &mut rng)
                .unwrap();
            let c = Weighted::evolver(Select::fill(Best), 0)
                .with_evolver(Select::fill(Worst), 1)
                .evolve((0, [0, 1, 2, 3, 4]), &mut rng)
                .unwrap();

            assert!(a == (1, [4, 4, 4, 4, 4]) || a == (1, [0, 0, 0, 0, 0]));
            assert_eq!(b, (1, [4, 4, 4, 4, 4]));
            assert_eq!(c, (1, [0, 0, 0, 0, 0]));
        }
    }

    #[test]
    fn test_score() {
        let mut rng = rand::rng();

        fn one(_: &i32) -> Result<i32, Infallible> {
            Ok(1)
        }

        fn two(_: &i32) -> Result<i32, Infallible> {
            Ok(2)
        }

        for _ in 0..10 {
            let a = Weighted::scorer(Function::new(one), 1)
                .with_scorer(Function::new(two), 1)
                .score(&10, &mut rng)
                .unwrap();
            let b = Weighted::scorer(Function::new(one), 1)
                .with_scorer(Function::new(two), 0)
                .score(&10, &mut rng)
                .unwrap();
            let c = Weighted::scorer(Function::new(one), 0)
                .with_scorer(Function::new(two), 1)
                .score(&10, &mut rng)
                .unwrap();

            assert!(a == 1 || a == 2);
            assert_eq!(b, 1);
            assert_eq!(c, 2);
        }
    }

    #[test]
    fn test_generate() {
        let mut rng = rand::rng();

        for _ in 0..10 {
            let a: u8 = Weighted::generator(Random::uniform(0..1), 1)
                .with_generator(Random::uniform(1..2), 1)
                .generate(&mut rng)
                .unwrap();
            let b: u8 = Weighted::generator(Random::uniform(0..1), 1)
                .with_generator(Random::uniform(1..2), 0)
                .generate(&mut rng)
                .unwrap();
            let c: u8 = Weighted::generator(Random::uniform(0..1), 0)
                .with_generator(Random::uniform(1..2), 1)
                .generate(&mut rng)
                .unwrap();

            assert!(a == 0 || a == 1);
            assert_eq!(b, 0);
            assert_eq!(c, 1);
        }
    }
}
