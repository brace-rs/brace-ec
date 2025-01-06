pub mod add;
pub mod invert;
pub mod noise;
pub mod rate;

use crate::core::fitness::FitnessMut;

use self::rate::Rate;

use super::inspect::Inspect;
use super::repeat::Repeat;
use super::score::Score;
use super::scorer::function::Function;
use super::scorer::Scorer;
use super::then::Then;

pub trait Mutator<T>: Sized {
    type Error;

    fn mutate<Rng>(&self, individual: T, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized;

    fn score<S>(self, scorer: S) -> Score<Self, S>
    where
        S: Scorer<T, Score = T::Value>,
        T: FitnessMut,
    {
        Score::new(self, scorer)
    }

    fn score_with<F, E>(self, scorer: F) -> Score<Self, Function<F>>
    where
        F: Fn(&T) -> Result<T::Value, E>,
        T: FitnessMut,
    {
        self.score(Function::new(scorer))
    }

    fn then<M>(self, mutator: M) -> Then<Self, M>
    where
        M: Mutator<T>,
    {
        Then::new(self, mutator)
    }

    fn rate(self, rate: f64) -> Rate<Self>
    where
        Self: Sized,
    {
        Rate::new(self, rate)
    }

    fn repeat(self, count: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, count)
    }

    fn inspect<F>(self, inspector: F) -> Inspect<Self, F>
    where
        F: Fn(&T),
        Self: Sized,
    {
        Inspect::new(self, inspector)
    }
}
