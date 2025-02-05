use thiserror::Error;

use crate::generation::Generation;
use crate::individual::Individual;
use crate::population::Population;
use crate::util::iter::IterableMut;
use crate::util::map::TryMap;

use super::evolver::Evolver;
use super::generator::Generator;
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

impl<P, T, S, I> Selector<P> for Score<T, S>
where
    P: Population<Individual = I> + ?Sized,
    T: Selector<P, Output: TryMap<Item = I>>,
    S: Scorer<I>,
    I: Individual,
{
    type Output = T::Output;
    type Error = ScoreError<T::Error, S::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
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
    S: Scorer<I>,
    I: Individual,
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

impl<P, T, S, I> Recombinator<P> for Score<T, S>
where
    P: Population<Individual = I>,
    T: Recombinator<P, Output: TryMap<Item = I>>,
    S: Scorer<I>,
    I: Individual,
{
    type Output = T::Output;
    type Error = ScoreError<T::Error, S::Error>;

    fn recombine<Rng>(&self, parents: P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
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

impl<G, T, S, P, I> Evolver<G> for Score<T, S>
where
    G: Generation<Population = P>,
    T: Evolver<G>,
    S: Scorer<I>,
    P: Population<Individual = I> + IterableMut<Item = I>,
    I: Individual,
{
    type Error = ScoreError<T::Error, S::Error>;

    fn evolve<Rng>(&self, generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let mut generation = self
            .operator
            .evolve(generation, rng)
            .map_err(ScoreError::Operate)?;

        generation
            .population_mut()
            .iter_mut()
            .try_for_each(|individual| {
                let fitness = self.scorer.score(individual, rng)?;

                individual.set_fitness(fitness);

                Ok(())
            })
            .map_err(ScoreError::Score)?;

        Ok(generation)
    }
}

impl<T, G, S> Generator<T> for Score<G, S>
where
    T: Individual,
    G: Generator<T>,
    S: Scorer<T>,
{
    type Error = ScoreError<G::Error, S::Error>;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let individual = self.operator.generate(rng).map_err(ScoreError::Operate)?;

        let fitness = self
            .scorer
            .score(&individual, rng)
            .map_err(ScoreError::Score)?;

        Ok(individual.with_fitness(fitness))
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

    use crate::individual::scored::Scored;
    use crate::individual::Individual;
    use crate::operator::evolver::select::Select;
    use crate::operator::evolver::Evolver;
    use crate::operator::generator::Generator;
    use crate::operator::mutator::add::Add;
    use crate::operator::mutator::Mutator;
    use crate::operator::recombinator::Recombinator;
    use crate::operator::scorer::function::Function;
    use crate::operator::selector::first::First;
    use crate::operator::selector::Selector;
    use crate::population::Population;

    fn double(individual: &Scored<i32, i32>) -> Result<i32, Infallible> {
        Ok(individual.individual * 2)
    }

    fn triple(individual: &Scored<i32, i32>) -> Result<i32, Infallible> {
        Ok(individual.individual * 3)
    }

    struct Noop;

    impl Recombinator<[Scored<i32, i32>; 2]> for Noop {
        type Output = [Scored<i32, i32>; 2];
        type Error = Infallible;

        fn recombine<Rng>(
            &self,
            parents: [Scored<i32, i32>; 2],
            _: &mut Rng,
        ) -> Result<Self::Output, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            Ok(parents)
        }
    }

    struct Make;

    impl Generator<Scored<i32, i32>> for Make {
        type Error = Infallible;

        fn generate<Rng>(&self, _: &mut Rng) -> Result<Scored<i32, i32>, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            Ok(Scored::new(100, 0))
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
            .mutated(Add(5).score(Function::new(double)))
            .unwrap();
        let b = Scored::new(10, 0)
            .mutated(Add(5).score(Function::new(triple)))
            .unwrap();
        let c = Scored::new(10, 0)
            .mutated(Add(5).score_with(|individual: &Scored<i32, i32>| {
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
            .recombined(Noop.score(Function::new(double)))
            .unwrap();
        let b = population
            .recombined(Noop.score(Function::new(triple)))
            .unwrap();
        let c = population
            .recombined(Noop.score_with(|individual: &Scored<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            }))
            .unwrap();

        assert_eq!(a, [Scored::new(10, 20), Scored::new(20, 40)]);
        assert_eq!(b, [Scored::new(10, 30), Scored::new(20, 60)]);
        assert_eq!(c, [Scored::new(10, 40), Scored::new(20, 80)]);
    }

    #[test]
    fn test_evolve() {
        let mut rng = rand::rng();

        let a = Select::fill(First)
            .score(Function::new(double))
            .evolve((0, [Scored::new(10, 0), Scored::new(20, 0)]), &mut rng)
            .unwrap();
        let b = Select::fill(First)
            .score(Function::new(triple))
            .evolve((0, [Scored::new(10, 0), Scored::new(20, 0)]), &mut rng)
            .unwrap();
        let c = Select::fill(First)
            .score_with(|individual: &Scored<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            })
            .evolve((0, [Scored::new(10, 0), Scored::new(20, 0)]), &mut rng)
            .unwrap();

        assert_eq!(a, (1, [Scored::new(10, 20), Scored::new(10, 20)]));
        assert_eq!(b, (1, [Scored::new(10, 30), Scored::new(10, 30)]));
        assert_eq!(c, (1, [Scored::new(10, 40), Scored::new(10, 40)]));
    }

    #[test]
    fn test_generate() {
        let mut rng = rand::rng();

        let a = Make
            .score(Function::new(double))
            .generate(&mut rng)
            .unwrap();
        let b = Make
            .score(Function::new(triple))
            .generate(&mut rng)
            .unwrap();
        let c = Make
            .score_with(|individual: &Scored<i32, i32>| {
                Ok::<_, Infallible>(individual.individual * 4)
            })
            .generate(&mut rng)
            .unwrap();

        assert_eq!(a, Scored::new(100, 200));
        assert_eq!(b, Scored::new(100, 300));
        assert_eq!(c, Scored::new(100, 400));
    }
}
