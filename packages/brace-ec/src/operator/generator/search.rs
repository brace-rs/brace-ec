use itertools::Itertools;
use thiserror::Error;

use crate::individual::Individual;

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

    use super::Search;

    #[test]
    fn test_generate() {
        let mut rng = rand::rng();

        let a = Search::new(Counter::u64(), 10).generate(&mut rng).unwrap();
        let b = Counter::u64().search(100).generate(&mut rng).unwrap();

        assert_eq!(a, 9);
        assert_eq!(b, 99);
    }
}
