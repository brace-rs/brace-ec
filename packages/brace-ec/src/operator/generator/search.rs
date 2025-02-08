use std::cmp::Ordering;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use thiserror::Error;

use crate::individual::Individual;
use crate::operator::IntoParallelOperator;

use super::Generator;

pub struct Search<G> {
    generator: G,
    iterations: usize,
}

impl<G> Search<G> {
    pub fn new(generator: G, iterations: usize) -> Self {
        Self {
            generator,
            iterations,
        }
    }
}

impl<T, G> Generator<T> for Search<G>
where
    T: Individual,
    G: Generator<T>,
{
    type Error = SearchError<G::Error>;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (0..self.iterations)
            .map(|_| self.generator.generate(rng))
            .process_results(|iter| iter.max_by(|a, b| a.fitness().cmp(b.fitness())))
            .map_err(SearchError::Generate)?
            .ok_or(SearchError::Zero)
    }
}

impl<G> IntoParallelOperator for Search<G> {
    type Op = ParSearch<G>;

    fn parallel(self) -> Self::Op {
        Self::Op {
            generator: self.generator,
            iterations: self.iterations,
        }
    }
}

pub struct ParSearch<G> {
    generator: G,
    iterations: usize,
}

impl<G> ParSearch<G> {
    pub fn new(generator: G, iterations: usize) -> Self {
        Self {
            generator,
            iterations,
        }
    }
}

impl<T, G> Generator<T> for ParSearch<G>
where
    T: Individual + Send,
    G: Generator<T, Error: Send> + Sync,
{
    type Error = SearchError<G::Error>;

    fn generate<Rng>(&self, _: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (0..self.iterations)
            .into_par_iter()
            .map_init(rand::rng, |rng, _| self.generator.generate(rng))
            .try_reduce_with(|a, b| match a.fitness().cmp(b.fitness()) {
                Ordering::Less => Ok(b),
                Ordering::Equal | Ordering::Greater => Ok(a),
            })
            .ok_or(SearchError::Zero)?
            .map_err(SearchError::Generate)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SearchError<G> {
    #[error("zero iterations")]
    Zero,
    #[error(transparent)]
    Generate(G),
}

#[cfg(test)]
mod tests {
    use crate::operator::generator::counter::Counter;
    use crate::operator::generator::Generator;
    use crate::operator::IntoParallelOperator;

    use super::{ParSearch, Search};

    #[test]
    fn test_generate_search() {
        let mut rng = rand::rng();

        let a = Search::new(Counter::u64(), 10).generate(&mut rng).unwrap();
        let b = Counter::u64().search(100).generate(&mut rng).unwrap();

        assert_eq!(a, 9);
        assert_eq!(b, 99);
    }

    #[test]
    fn test_generate_par_search() {
        let mut rng = rand::rng();

        let a = ParSearch::new(Counter::u64(), 10)
            .generate(&mut rng)
            .unwrap();
        let b = Counter::u64().par_search(11).generate(&mut rng).unwrap();
        let c = Counter::u64()
            .search(11)
            .parallel()
            .generate(&mut rng)
            .unwrap();

        assert_eq!(a, 9);
        assert_eq!(b, 10);
        assert_eq!(c, 10);
    }
}
