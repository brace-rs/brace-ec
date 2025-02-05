use std::iter::repeat_with;
use std::marker::PhantomData;

use thiserror::Error;

use crate::population::Population;
use crate::util::iter::TryFromIterator;

use super::Generator;

pub struct Populate<T, P> {
    generator: T,
    size: usize,
    marker: PhantomData<fn() -> P>,
}

impl<T, P> Populate<T, P> {
    pub fn new(generator: T, size: usize) -> Self {
        Self {
            generator,
            size,
            marker: PhantomData,
        }
    }
}

impl<P, T> Generator<P> for Populate<T, P>
where
    P: Population + TryFromIterator<P::Individual>,
    T: Generator<P::Individual>,
{
    type Error = PopulateError<P::Error, T::Error>;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<P, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let iter = repeat_with(|| self.generator.generate(rng)).take(self.size);

        Result::try_from_iter(iter)
            .map_err(PopulateError::Collect)?
            .map_err(PopulateError::Generate)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PopulateError<C, G> {
    #[error(transparent)]
    Collect(C),
    #[error(transparent)]
    Generate(G),
}

#[cfg(test)]
mod tests {
    use crate::operator::generator::random::Random;
    use crate::operator::generator::Generator;

    #[test]
    fn test_generate() {
        let mut rng = rand::rng();

        let a: Vec<u8> = Random::from(1..2).populate(5).generate(&mut rng).unwrap();
        let b: [u8; 5] = Random::from(1..2).populate(5).generate(&mut rng).unwrap();
        let c: [[u8; 2]; 3] = Random::from(1..2)
            .populate(2)
            .populate(3)
            .generate(&mut rng)
            .unwrap();

        assert_eq!(a, [1; 5]);
        assert_eq!(b, [1; 5]);
        assert_eq!(c, [[1; 2]; 3]);
    }
}
