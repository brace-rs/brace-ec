use thiserror::Error;

use crate::generation::Generation;
use crate::individual::Individual;
use crate::population::Population;
use crate::util::iter::IterableMut;
use crate::util::map::TryMap;

use super::evaluator::Evaluator;
use super::evolver::Evolver;
use super::generator::Generator;
use super::mutator::Mutator;
use super::recombinator::Recombinator;
use super::selector::Selector;

pub struct Evaluate<T, S> {
    operator: T,
    evaluator: S,
}

impl<T, S> Evaluate<T, S> {
    pub fn new(operator: T, evaluator: S) -> Self {
        Self {
            operator,
            evaluator,
        }
    }
}

impl<P, T, S, I> Selector<P> for Evaluate<T, S>
where
    P: Population<Individual = I> + ?Sized,
    T: Selector<P, Output: TryMap<Item = I>>,
    S: Evaluator<I>,
    I: Individual,
{
    type Output = T::Output;
    type Error = EvaluateError<T::Error, S::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operator
            .select(population, rng)
            .map_err(EvaluateError::Operate)?
            .try_map(|individual| {
                let fitness = self.evaluator.evaluate(&individual, rng)?;

                Ok(individual.with_fitness(fitness))
            })
            .map_err(EvaluateError::Evaluate)
    }
}

impl<T, S, I> Mutator<I> for Evaluate<T, S>
where
    T: Mutator<I>,
    S: Evaluator<I>,
    I: Individual,
{
    type Error = EvaluateError<T::Error, S::Error>;

    fn mutate<Rng>(&self, individual: I, rng: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let individual = self
            .operator
            .mutate(individual, rng)
            .map_err(EvaluateError::Operate)?;

        let fitness = self
            .evaluator
            .evaluate(&individual, rng)
            .map_err(EvaluateError::Evaluate)?;

        Ok(individual.with_fitness(fitness))
    }
}

impl<P, T, S, I> Recombinator<P> for Evaluate<T, S>
where
    P: Population<Individual = I>,
    T: Recombinator<P, Output: TryMap<Item = I>>,
    S: Evaluator<I>,
    I: Individual,
{
    type Output = T::Output;
    type Error = EvaluateError<T::Error, S::Error>;

    fn recombine<Rng>(&self, parents: P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.operator
            .recombine(parents, rng)
            .map_err(EvaluateError::Operate)?
            .try_map(|individual| {
                let fitness = self.evaluator.evaluate(&individual, rng)?;

                Ok(individual.with_fitness(fitness))
            })
            .map_err(EvaluateError::Evaluate)
    }
}

impl<G, T, S, P, I> Evolver<G> for Evaluate<T, S>
where
    G: Generation<Population = P>,
    T: Evolver<G>,
    S: Evaluator<I>,
    P: Population<Individual = I> + IterableMut<Item = I>,
    I: Individual,
{
    type Error = EvaluateError<T::Error, S::Error>;

    fn evolve<Rng>(&self, generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut generation = self
            .operator
            .evolve(generation, rng)
            .map_err(EvaluateError::Operate)?;

        generation
            .population_mut()
            .iter_mut()
            .try_for_each(|individual| {
                let fitness = self.evaluator.evaluate(individual, rng)?;

                individual.set_fitness(fitness);

                Ok(())
            })
            .map_err(EvaluateError::Evaluate)?;

        Ok(generation)
    }
}

impl<T, G, S> Generator<T> for Evaluate<G, S>
where
    T: Individual,
    G: Generator<T>,
    S: Evaluator<T>,
{
    type Error = EvaluateError<G::Error, S::Error>;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let individual = self
            .operator
            .generate(rng)
            .map_err(EvaluateError::Operate)?;

        let fitness = self
            .evaluator
            .evaluate(&individual, rng)
            .map_err(EvaluateError::Evaluate)?;

        Ok(individual.with_fitness(fitness))
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EvaluateError<O, S> {
    #[error(transparent)]
    Operate(O),
    #[error(transparent)]
    Evaluate(S),
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::individual::evaluated::Evaluated;
    use crate::individual::Individual;
    use crate::operator::evaluator::function::Function;
    use crate::operator::evolver::select::Select;
    use crate::operator::evolver::Evolver;
    use crate::operator::generator::Generator;
    use crate::operator::mutator::add::Add;
    use crate::operator::mutator::Mutator;
    use crate::operator::recombinator::Recombinator;
    use crate::operator::selector::first::First;
    use crate::operator::selector::Selector;
    use crate::population::Population;

    fn double(individual: &Evaluated<i32, i32>) -> Result<i32, Infallible> {
        Ok(individual.individual * 2)
    }

    fn triple(individual: &Evaluated<i32, i32>) -> Result<i32, Infallible> {
        Ok(individual.individual * 3)
    }

    struct Noop;

    impl Recombinator<[Evaluated<i32, i32>; 2]> for Noop {
        type Output = [Evaluated<i32, i32>; 2];
        type Error = Infallible;

        fn recombine<Rng>(
            &self,
            parents: [Evaluated<i32, i32>; 2],
            _: &mut Rng,
        ) -> Result<Self::Output, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            Ok(parents)
        }
    }

    struct Make;

    impl Generator<Evaluated<i32, i32>> for Make {
        type Error = Infallible;

        fn generate<Rng>(&self, _: &mut Rng) -> Result<Evaluated<i32, i32>, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            Ok(Evaluated::new(100, 0))
        }
    }

    #[test]
    fn test_select() {
        let population = [Evaluated::new(10, 0)];

        let a = population
            .select(First.evaluate(Function::new(double)))
            .unwrap()[0];
        let b = population
            .select(First.evaluate(Function::new(triple)))
            .unwrap()[0];
        let c = population
            .select(First.evaluate_with(|individual: &Evaluated<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            }))
            .unwrap()[0];

        assert_eq!(a, Evaluated::new(10, 20));
        assert_eq!(b, Evaluated::new(10, 30));
        assert_eq!(c, Evaluated::new(10, 40));
    }

    #[test]
    fn test_mutate() {
        let a = Evaluated::new(10, 0)
            .mutated(Add(5).evaluate(Function::new(double)))
            .unwrap();
        let b = Evaluated::new(10, 0)
            .mutated(Add(5).evaluate(Function::new(triple)))
            .unwrap();
        let c = Evaluated::new(10, 0)
            .mutated(Add(5).evaluate_with(|individual: &Evaluated<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            }))
            .unwrap();

        assert_eq!(a, Evaluated::new(15, 30));
        assert_eq!(b, Evaluated::new(15, 45));
        assert_eq!(c, Evaluated::new(15, 60));
    }

    #[test]
    fn test_recombine() {
        let population = [Evaluated::new(10, 0), Evaluated::new(20, 0)];

        let a = population
            .recombined(Noop.evaluate(Function::new(double)))
            .unwrap();
        let b = population
            .recombined(Noop.evaluate(Function::new(triple)))
            .unwrap();
        let c = population
            .recombined(Noop.evaluate_with(|individual: &Evaluated<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            }))
            .unwrap();

        assert_eq!(a, [Evaluated::new(10, 20), Evaluated::new(20, 40)]);
        assert_eq!(b, [Evaluated::new(10, 30), Evaluated::new(20, 60)]);
        assert_eq!(c, [Evaluated::new(10, 40), Evaluated::new(20, 80)]);
    }

    #[test]
    fn test_evolve() {
        let mut rng = rand::rng();

        let a = Select::fill(First)
            .evaluate(Function::new(double))
            .evolve(
                (0, [Evaluated::new(10, 0), Evaluated::new(20, 0)]),
                &mut rng,
            )
            .unwrap();
        let b = Select::fill(First)
            .evaluate(Function::new(triple))
            .evolve(
                (0, [Evaluated::new(10, 0), Evaluated::new(20, 0)]),
                &mut rng,
            )
            .unwrap();
        let c = Select::fill(First)
            .evaluate_with(|individual: &Evaluated<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            })
            .evolve(
                (0, [Evaluated::new(10, 0), Evaluated::new(20, 0)]),
                &mut rng,
            )
            .unwrap();

        assert_eq!(a, (1, [Evaluated::new(10, 20), Evaluated::new(10, 20)]));
        assert_eq!(b, (1, [Evaluated::new(10, 30), Evaluated::new(10, 30)]));
        assert_eq!(c, (1, [Evaluated::new(10, 40), Evaluated::new(10, 40)]));
    }

    #[test]
    fn test_generate() {
        let mut rng = rand::rng();

        let a = Make
            .evaluate(Function::new(double))
            .generate(&mut rng)
            .unwrap();
        let b = Make
            .evaluate(Function::new(triple))
            .generate(&mut rng)
            .unwrap();
        let c = Make
            .evaluate_with(|individual: &Evaluated<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            })
            .generate(&mut rng)
            .unwrap();

        assert_eq!(a, Evaluated::new(100, 200));
        assert_eq!(b, Evaluated::new(100, 300));
        assert_eq!(c, Evaluated::new(100, 400));
    }
}
