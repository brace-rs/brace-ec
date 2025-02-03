pub mod counter;
pub mod populate;
pub mod random;
pub mod search;

use crate::core::individual::Individual;
use crate::core::population::Population;
use crate::util::iter::TryFromIterator;

use self::populate::Populate;
use self::search::Search;

use super::score::Score;
use super::scorer::function::Function;
use super::scorer::Scorer;
use super::selector::generate::Generate;

pub trait Generator<T>: Sized {
    type Error;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized;

    fn populate<P>(self, size: usize) -> Populate<Self, P>
    where
        P: Population<Individual = T> + TryFromIterator<T>,
    {
        Populate::new(self, size)
    }

    fn score<S>(self, scorer: S) -> Score<Self, S>
    where
        S: Scorer<T, Score = T::Fitness>,
        T: Individual,
    {
        Score::new(self, scorer)
    }

    fn score_with<F, E>(self, scorer: F) -> Score<Self, Function<F>>
    where
        F: Fn(&T) -> Result<T::Fitness, E>,
        T: Individual,
    {
        self.score(Function::new(scorer))
    }

    fn search(self, iterations: usize) -> Search<Self>
    where
        T: Individual,
    {
        Search::new(self, iterations)
    }

    fn selector<P>(self) -> Generate<Self, P>
    where
        P: Population<Individual = T> + ?Sized,
    {
        Generate::new(self)
    }
}

pub trait DynGenerator<T, E = Box<dyn std::error::Error>> {
    fn dyn_generate(&self, rng: &mut dyn rand::RngCore) -> Result<T, E>;
}

impl<T, E, G> DynGenerator<T, E> for G
where
    G: Generator<T, Error: Into<E>>,
{
    fn dyn_generate(&self, rng: &mut dyn rand::RngCore) -> Result<T, E> {
        self.generate(rng).map_err(Into::into)
    }
}

impl<T, E> Generator<T> for Box<dyn DynGenerator<T, E>> {
    type Error = E;

    fn generate<Rng>(&self, mut rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        (**self).dyn_generate(&mut rng)
    }
}
