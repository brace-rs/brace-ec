use thiserror::Error;

use crate::individual::Individual;
use crate::operator::evaluate::Evaluate;
use crate::operator::evaluator::Evaluator;
use crate::operator::mutator::Mutator;
use crate::population::Population;

use super::Selector;

pub struct HillClimb<S, M> {
    selector: S,
    mutator: M,
    iterations: usize,
}

impl<S, M> HillClimb<S, M> {
    pub fn new(selector: S, mutator: M, iterations: usize) -> Self {
        Self {
            selector,
            mutator,
            iterations,
        }
    }
}

impl<S, M> HillClimb<S, M> {
    pub fn evaluate<T, P>(self, evaluator: T) -> HillClimb<Evaluate<S, T>, Evaluate<M, T>>
    where
        P: Population<Individual: Clone> + ?Sized,
        T: Evaluator<P::Individual> + Clone,
        S: Selector<P, Output = [P::Individual; 1]>,
        M: Mutator<P::Individual>,
    {
        HillClimb {
            selector: self.selector.evaluate(evaluator.clone()),
            mutator: self.mutator.evaluate(evaluator),
            iterations: self.iterations,
        }
    }
}

impl<P, S, M> Selector<P> for HillClimb<S, M>
where
    P: Population<Individual: Clone> + ?Sized,
    S: Selector<P, Output = [P::Individual; 1]>,
    M: Mutator<P::Individual>,
{
    type Output = [P::Individual; 1];
    type Error = HillClimbError<S::Error, M::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let [individual] = self
            .selector
            .select(population, rng)
            .map_err(HillClimbError::Select)?;

        let individual = (0..self.iterations)
            .try_fold(individual, |prev, _| {
                let next = self.mutator.mutate(prev.clone(), rng)?;

                if next.fitness() > prev.fitness() {
                    Ok(next)
                } else {
                    Ok(prev)
                }
            })
            .map_err(HillClimbError::Mutate)?;

        Ok([individual])
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HillClimbError<S, M> {
    #[error(transparent)]
    Select(S),
    #[error(transparent)]
    Mutate(M),
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::operator::evaluator::Evaluator;
    use crate::operator::mutator::add::Add;
    use crate::operator::mutator::Mutator;
    use crate::operator::selector::best::Best;
    use crate::operator::selector::Selector;

    use super::HillClimb;

    #[derive(Clone)]
    struct HillEvaluator;

    impl Evaluator<i32> for HillEvaluator {
        type Error = Infallible;

        fn evaluate<Rng>(&self, input: &i32, _: &mut Rng) -> Result<i32, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            match input {
                10 => Ok(0),
                _ => Ok(*input),
            }
        }
    }

    #[test]
    fn test_select() {
        let mut rng = rand::rng();

        let a = HillClimb::new(Best, Add(1), 10)
            .select(&[1, 2, 3, 4, 5], &mut rng)
            .unwrap();
        let b = Best
            .hill_climb(Add(1), 10)
            .select(&[1, 2, 3, 4, 5], &mut rng)
            .unwrap();
        let c = Best
            .evaluate(HillEvaluator)
            .hill_climb(Add(1).evaluate(HillEvaluator), 10)
            .select(&[1, 2, 3, 4, 5], &mut rng)
            .unwrap();
        let d = Best
            .hill_climb(Add(1), 10)
            .evaluate(HillEvaluator)
            .select(&[1, 2, 3, 4, 5], &mut rng)
            .unwrap();

        assert_eq!(a, [15]);
        assert_eq!(b, [15]);
        assert_eq!(c, [9]);
        assert_eq!(d, [9]);
    }
}
