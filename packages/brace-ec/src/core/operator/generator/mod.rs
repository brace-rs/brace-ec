pub mod populate;
pub mod random;

use crate::core::fitness::FitnessMut;
use crate::core::population::Population;
use crate::util::iter::TryFromIterator;

use self::populate::Populate;

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

    fn selector<P>(self) -> Generate<Self, P>
    where
        P: Population<Individual = T>,
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

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::convert::Infallible;

    use super::Generator;

    struct Count(Cell<u8>);

    impl Generator<u8> for Count {
        type Error = Infallible;

        fn generate<Rng>(&self, _: &mut Rng) -> Result<u8, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            let n = self.0.get() + 1;

            self.0.set(n);

            Ok(n)
        }
    }

    #[test]
    fn test_generate() {
        let mut rng = rand::rng();

        let count = Count(Cell::new(0));

        let a = count.generate(&mut rng).unwrap();
        let b = count.generate(&mut rng).unwrap();
        let c = count.generate(&mut rng).unwrap();

        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
    }
}
