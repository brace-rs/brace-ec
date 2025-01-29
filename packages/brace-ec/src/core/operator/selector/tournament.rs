use std::marker::PhantomData;

use rand::seq::IteratorRandom;
use thiserror::Error;

use crate::core::individual::Individual;
use crate::core::population::{IterablePopulation, Population};

use super::Selector;

pub struct Tournament<P: Population + ?Sized> {
    size: usize,
    marker: PhantomData<fn() -> P>,
}

impl<P> Tournament<P>
where
    P: Population + ?Sized,
{
    pub fn new(size: usize) -> Self {
        Self {
            size,
            marker: PhantomData,
        }
    }

    pub fn binary() -> Self {
        Self::new(2)
    }
}

impl<P> Selector<P> for Tournament<P>
where
    P: IterablePopulation<Individual: Clone> + ?Sized,
{
    type Output = [P::Individual; 1];
    type Error = TournamentError;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if self.size == 0 {
            return Err(TournamentError::Empty);
        }

        if population.len() < self.size {
            return Err(TournamentError::NotEnough);
        }

        Ok([population
            .iter()
            .choose_multiple(rng, self.size)
            .into_iter()
            .max_by_key(|individual| individual.fitness())
            .expect("bound check")
            .clone()])
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TournamentError {
    #[error("empty tournament")]
    Empty,
    #[error("not enough participants")]
    NotEnough,
}

#[cfg(test)]
mod tests {
    use crate::core::population::Population;

    use super::{Tournament, TournamentError};

    #[test]
    fn test_select() {
        let population = [0, 1, 2, 3, 4];

        let a = population.select(Tournament::new(1)).unwrap();
        let b = population.select(Tournament::new(2)).unwrap();
        let c = population.select(Tournament::new(3)).unwrap();
        let d = population.select(Tournament::new(4)).unwrap();
        let e = population.select(Tournament::new(5)).unwrap();
        let f = population.select(Tournament::new(6));
        let g = population.select(Tournament::new(0));

        assert!(population.contains(&a[0]));
        assert!(population.contains(&b[0]));
        assert!(population.contains(&c[0]));
        assert!(population.contains(&d[0]));

        assert_eq!(e, [4]);
        assert_eq!(f, Err(TournamentError::NotEnough));
        assert_eq!(g, Err(TournamentError::Empty));
    }
}
