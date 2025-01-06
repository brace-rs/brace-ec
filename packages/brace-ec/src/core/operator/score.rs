use rand::{thread_rng, Rng};
use thiserror::Error;

use crate::core::fitness::FitnessMut;
use crate::core::generation::Generation;
use crate::core::population::Population;
use crate::util::iter::IterableMut;
use crate::util::map::TryMap;

use super::evolver::Evolver;
use super::mutator::Mutator;
use super::recombinator::Recombinator;
use super::scorer::Scorer;
use super::selector::Selector;

pub struct Score<T, S> {
    operator: T,
    scorer: S,
}

impl<T, S> Score<T, S> {
    pub fn new(operator: T, scorer: S) -> Self {
        Self { operator, scorer }
    }
}

impl<T, S, I> Selector for Score<T, S>
where
    T: Selector<Population: Population<Individual = I>, Output: TryMap<Item = I>>,
    S: Scorer<I, Score = I::Value>,
    I: FitnessMut,
{
    type Population = T::Population;
    type Output = T::Output;
    type Error = ScoreError<T::Error, S::Error>;

    fn select<R>(
        &self,
        population: &Self::Population,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>
    where
        R: Rng + ?Sized,
    {
        self.operator
            .select(population, rng)
            .map_err(ScoreError::Operate)?
            .try_map(|individual| {
                let fitness = self.scorer.score(&individual, rng)?;

                Ok(individual.with_fitness(fitness))
            })
            .map_err(ScoreError::Score)
    }
}

impl<T, S, I> Mutator<I> for Score<T, S>
where
    T: Mutator<I>,
    S: Scorer<I, Score = I::Value>,
    I: FitnessMut,
{
    type Error = ScoreError<T::Error, S::Error>;

    fn mutate<Rng>(&self, individual: I, rng: &mut Rng) -> Result<I, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let individual = self
            .operator
            .mutate(individual, rng)
            .map_err(ScoreError::Operate)?;

        let fitness = self
            .scorer
            .score(&individual, rng)
            .map_err(ScoreError::Score)?;

        Ok(individual.with_fitness(fitness))
    }
}

impl<T, S, I> Recombinator for Score<T, S>
where
    T: Recombinator<Parents: Population<Individual = I>, Output: TryMap<Item = I>>,
    S: Scorer<I, Score = I::Value>,
    I: FitnessMut,
{
    type Parents = T::Parents;
    type Output = T::Output;
    type Error = ScoreError<T::Error, S::Error>;

    fn recombine<R>(&self, parents: Self::Parents, rng: &mut R) -> Result<Self::Output, Self::Error>
    where
        R: Rng + ?Sized,
    {
        self.operator
            .recombine(parents, rng)
            .map_err(ScoreError::Operate)?
            .try_map(|individual| {
                let fitness = self.scorer.score(&individual, rng)?;

                Ok(individual.with_fitness(fitness))
            })
            .map_err(ScoreError::Score)
    }
}

impl<T, S, P, I> Evolver for Score<T, S>
where
    T: Evolver<Generation: Generation<Population = P>>,
    S: Scorer<I, Score = I::Value>,
    P: Population<Individual = I> + IterableMut<Item = I>,
    I: FitnessMut,
{
    type Generation = T::Generation;
    type Error = ScoreError<T::Error, S::Error>;

    fn evolve(&self, generation: Self::Generation) -> Result<Self::Generation, Self::Error> {
        let mut rng = thread_rng();
        let mut generation = self
            .operator
            .evolve(generation)
            .map_err(ScoreError::Operate)?;

        generation
            .population_mut()
            .iter_mut()
            .try_for_each(|individual| {
                let fitness = self.scorer.score(individual, &mut rng)?;

                individual.set_fitness(fitness);

                Ok(())
            })
            .map_err(ScoreError::Score)?;

        Ok(generation)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScoreError<O, S> {
    #[error(transparent)]
    Operate(O),
    #[error(transparent)]
    Score(S),
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use rand::Rng;

    use crate::core::individual::scored::Scored;
    use crate::core::individual::Individual;
    use crate::core::operator::evolver::select::Select;
    use crate::core::operator::evolver::Evolver;
    use crate::core::operator::mutator::add::Add;
    use crate::core::operator::mutator::Mutator;
    use crate::core::operator::recombinator::Recombinator;
    use crate::core::operator::scorer::function::Function;
    use crate::core::operator::selector::first::First;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    fn double(individual: &Scored<i32, i32>) -> Result<i32, Infallible> {
        Ok(individual.individual * 2)
    }

    fn triple(individual: &Scored<i32, i32>) -> Result<i32, Infallible> {
        Ok(individual.individual * 3)
    }

    struct Noop;

    impl Recombinator for Noop {
        type Parents = [Scored<i32, i32>; 2];
        type Output = [Scored<i32, i32>; 2];
        type Error = Infallible;

        fn recombine<R>(
            &self,
            parents: Self::Parents,
            _: &mut R,
        ) -> Result<Self::Output, Self::Error>
        where
            R: Rng + ?Sized,
        {
            Ok(parents)
        }
    }

    #[test]
    fn test_select() {
        let population = [Scored::new(10, 0)];

        let a = population
            .select(First.score(Function::new(double)))
            .unwrap()[0];
        let b = population
            .select(First.score(Function::new(triple)))
            .unwrap()[0];
        let c = population
            .select(First.score_with(|individual: &Scored<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            }))
            .unwrap()[0];

        assert_eq!(a, Scored::new(10, 20));
        assert_eq!(b, Scored::new(10, 30));
        assert_eq!(c, Scored::new(10, 40));
    }

    #[test]
    fn test_mutate() {
        let a = Scored::new(10, 0)
            .mutate(Add(5).score(Function::new(double)))
            .unwrap();
        let b = Scored::new(10, 0)
            .mutate(Add(5).score(Function::new(triple)))
            .unwrap();
        let c = Scored::new(10, 0)
            .mutate(Add(5).score_with(|individual: &Scored<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            }))
            .unwrap();

        assert_eq!(a, Scored::new(15, 30));
        assert_eq!(b, Scored::new(15, 45));
        assert_eq!(c, Scored::new(15, 60));
    }

    #[test]
    fn test_recombine() {
        let population = [Scored::new(10, 0), Scored::new(20, 0)];

        let a = population
            .recombine(Noop.score(Function::new(double)))
            .unwrap();
        let b = population
            .recombine(Noop.score(Function::new(triple)))
            .unwrap();
        let c = population
            .recombine(Noop.score_with(|individual: &Scored<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            }))
            .unwrap();

        assert_eq!(a, [Scored::new(10, 20), Scored::new(20, 40)]);
        assert_eq!(b, [Scored::new(10, 30), Scored::new(20, 60)]);
        assert_eq!(c, [Scored::new(10, 40), Scored::new(20, 80)]);
    }

    #[test]
    fn test_evolve() {
        let a = Select::new(First)
            .score(Function::new(double))
            .evolve((0, [Scored::new(10, 0), Scored::new(20, 0)]))
            .unwrap();
        let b = Select::new(First)
            .score(Function::new(triple))
            .evolve((0, [Scored::new(10, 0), Scored::new(20, 0)]))
            .unwrap();
        let c = Select::new(First)
            .score_with(|individual: &Scored<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            })
            .evolve((0, [Scored::new(10, 0), Scored::new(20, 0)]))
            .unwrap();

        assert_eq!(a, (1, [Scored::new(10, 20), Scored::new(10, 20)]));
        assert_eq!(b, (1, [Scored::new(10, 30), Scored::new(10, 30)]));
        assert_eq!(c, (1, [Scored::new(10, 40), Scored::new(10, 40)]));
    }
}
