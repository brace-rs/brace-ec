use std::marker::PhantomData;

use crate::generation::Generation;
use crate::operator::selector::fill::{Fill, ParFill};
use crate::operator::selector::Selector;
use crate::population::Population;

use super::Evolver;

#[derive(Clone, Debug, Default)]
pub struct Select<S, G> {
    selector: S,
    marker: PhantomData<fn() -> G>,
}

impl<S, G> Select<S, G> {
    pub fn new(selector: S) -> Self {
        Self {
            selector,
            marker: PhantomData,
        }
    }
}

impl<S, G> Select<Fill<S>, G> {
    pub fn fill(selector: S) -> Self {
        Self {
            selector: Fill::new(selector),
            marker: PhantomData,
        }
    }
}

impl<S, G> Select<ParFill<S>, G> {
    pub fn par_fill(selector: S) -> Self {
        Self {
            selector: ParFill::new(selector),
            marker: PhantomData,
        }
    }
}

impl<P, G, S> Evolver<G> for Select<S, G>
where
    P: Population,
    G: Generation<Population = P>,
    S: Selector<P, Output: Into<P>>,
{
    type Error = S::Error;

    fn evolve<Rng>(&self, generation: G, rng: &mut Rng) -> Result<G, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let population = self.selector.select(generation.population(), rng)?;

        Ok(generation.advanced_with(population))
    }
}

#[cfg(test)]
mod tests {
    use crate::operator::evolver::Evolver;
    use crate::operator::selector::random::Random;
    use crate::operator::selector::Selector;

    use super::Select;

    #[test]
    fn test_evolve() {
        let mut rng = rand::rng();

        let evolver = Select::fill(Random);
        let population = [0, 1, 2, 3, 4];
        let generation = evolver.evolve((0, population), &mut rng).unwrap();

        assert_eq!(generation.0, 1);
        assert!(generation.1.iter().all(|i| population.contains(i)));

        let generation = evolver.evolve(generation, &mut rng).unwrap();

        assert_eq!(generation.0, 2);
        assert!(generation.1.iter().all(|i| population.contains(i)));

        let population = [1, 2, 3, 4, 5];
        let generation = Random
            .fill()
            .evolver()
            .repeat(2)
            .evolve((0, population), &mut rng)
            .unwrap();

        assert_eq!(generation.0, 2);
        assert_eq!(generation.1.len(), 5);
        assert!(generation.1.iter().all(|i| population.contains(i)));
    }
}
