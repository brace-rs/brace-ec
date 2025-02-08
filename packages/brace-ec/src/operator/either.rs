use thiserror::Error;

use crate::generation::Generation;
use crate::individual::Individual;
use crate::population::Population;

use super::evaluator::Evaluator;
use super::evolver::Evolver;
use super::generator::Generator;
use super::mutator::Mutator;
use super::recombinator::Recombinator;
use super::selector::Selector;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Either<A, B> {
    #[error(transparent)]
    A(A),
    #[error(transparent)]
    B(B),
}

impl<A, B> Either<A, B> {
    pub fn a(a: A) -> Self {
        Self::A(a)
    }

    pub fn b(b: B) -> Self {
        Self::B(b)
    }
}

impl<P, A, B> Selector<P> for Either<A, B>
where
    P: Population + ?Sized,
    A: Selector<P>,
    B: Selector<P, Output = A::Output>,
{
    type Output = A::Output;
    type Error = Either<A::Error, B::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        match self {
            Self::A(a) => a.select(population, rng).map_err(Either::A),
            Self::B(b) => b.select(population, rng).map_err(Either::B),
        }
    }
}

impl<T, A, B> Mutator<T> for Either<A, B>
where
    T: Individual,
    A: Mutator<T>,
    B: Mutator<T>,
{
    type Error = Either<A::Error, B::Error>;

    fn mutate<Rng>(&self, individual: T, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        match self {
            Self::A(a) => a.mutate(individual, rng).map_err(Either::A),
            Self::B(b) => b.mutate(individual, rng).map_err(Either::B),
        }
    }
}

impl<P, A, B> Recombinator<P> for Either<A, B>
where
    P: Population,
    A: Recombinator<P>,
    B: Recombinator<P, Output = A::Output>,
{
    type Output = A::Output;
    type Error = Either<A::Error, B::Error>;

    fn recombine<Rng>(&self, parents: P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        match self {
            Self::A(a) => a.recombine(parents, rng).map_err(Either::A),
            Self::B(b) => b.recombine(parents, rng).map_err(Either::B),
        }
    }
}

impl<G, A, B> Evolver<G> for Either<A, B>
where
    G: Generation,
    A: Evolver<G>,
    B: Evolver<G>,
{
    type Error = Either<A::Error, B::Error>;

    fn evolve<Rng>(&self, generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        match self {
            Self::A(a) => a.evolve(generation, rng).map_err(Either::A),
            Self::B(b) => b.evolve(generation, rng).map_err(Either::B),
        }
    }
}

impl<T, A, B> Generator<T> for Either<A, B>
where
    A: Generator<T>,
    B: Generator<T>,
{
    type Error = Either<A::Error, B::Error>;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        match self {
            Self::A(a) => a.generate(rng).map_err(Either::A),
            Self::B(b) => b.generate(rng).map_err(Either::B),
        }
    }
}

impl<T, A, B> Evaluator<T> for Either<A, B>
where
    T: Individual,
    A: Evaluator<T>,
    B: Evaluator<T>,
{
    type Error = Either<A::Error, B::Error>;

    fn evaluate<Rng>(&self, individual: &T, rng: &mut Rng) -> Result<T::Fitness, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        match self {
            Self::A(a) => a.evaluate(individual, rng).map_err(Either::A),
            Self::B(b) => b.evaluate(individual, rng).map_err(Either::B),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::individual::Individual;
    use crate::operator::evaluator::count::Count;
    use crate::operator::evaluator::Evaluator;
    use crate::operator::evolver::Evolver;
    use crate::operator::generator::counter::Counter;
    use crate::operator::generator::Generator;
    use crate::operator::mutator::add::Add;
    use crate::operator::recombinator::average::Average;
    use crate::operator::recombinator::sum::Sum;
    use crate::operator::selector::best::Best;
    use crate::operator::selector::worst::Worst;
    use crate::operator::selector::Selector;
    use crate::population::Population;

    use super::Either;

    #[test]
    fn test_select() {
        for (which, val) in [(true, 3), (false, 1)] {
            let selector = match which {
                true => Either::a(Best),
                false => Either::b(Worst),
            };

            let output = [1, 2, 3].select(selector).unwrap();

            assert_eq!(output[0], val);
        }
    }

    #[test]
    fn test_mutate() {
        for (which, val) in [(true, 2), (false, 3)] {
            let mutator = match which {
                true => Either::a(Add(1)),
                false => Either::b(Add(2)),
            };

            let output = 1.mutated(mutator).unwrap();

            assert_eq!(output, val);
        }
    }

    #[test]
    fn test_recombine() {
        for (which, val) in [(true, 3), (false, 6)] {
            let recombinator = match which {
                true => Either::a(Average),
                false => Either::b(Sum),
            };

            let output = [1, 5].recombined(recombinator).unwrap();

            assert_eq!(output[0], val);
        }
    }

    #[test]
    fn test_evolve() {
        let mut rng = rand::rng();

        for (which, val) in [(true, 3), (false, 1)] {
            let evolver = match which {
                true => Either::a(Best.fill().evolver()),
                false => Either::b(Worst.fill().evolver()),
            };

            let output = evolver.evolve((0, [1, 2, 3]), &mut rng).unwrap();

            assert_eq!(output.0, 1);
            assert_eq!(output.1, [val; 3]);
        }
    }

    #[test]
    fn test_generate() {
        let mut rng = rand::rng();

        for (which, val) in [(true, 1), (false, 3)] {
            let generator = match which {
                true => Either::a(Counter::u64().search(2)),
                false => Either::b(Counter::u64().search(4)),
            };

            let output = generator.generate(&mut rng).unwrap();

            assert_eq!(output, val);
        }
    }

    #[test]
    fn test_evaluate() {
        let mut rng = rand::rng();

        for (which, val) in [(true, 1), (false, 3)] {
            let evaluator = match which {
                true => Either::a(Count::new::<u8>(true)),
                false => Either::b(Count::new::<u8>(false)),
            };

            let output = evaluator
                .evaluate(&[false, true, false, false].evaluated::<u8>(), &mut rng)
                .unwrap();

            assert_eq!(output, val);
        }
    }
}
