use std::marker::PhantomData;

use crate::operator::generator::Generator;
use crate::population::Population;

use super::Selector;

pub struct Generate<T, P>
where
    P: ?Sized,
{
    generator: T,
    marker: PhantomData<fn() -> P>,
}

impl<T, P> Generate<T, P>
where
    P: ?Sized,
{
    pub fn new(generator: T) -> Self {
        Self {
            generator,
            marker: PhantomData,
        }
    }
}

impl<P, T> Selector<P> for Generate<T, P>
where
    P: Population + ?Sized,
    T: Generator<P::Individual>,
{
    type Output = [P::Individual; 1];
    type Error = T::Error;

    fn select<Rng>(&self, _: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok([self.generator.generate(rng)?])
    }
}

#[cfg(test)]
mod tests {
    use crate::operator::generator::random::Random;
    use crate::operator::generator::Generator;
    use crate::population::Population;

    #[test]
    fn test_select() {
        let population = [1, 2, 3, 4, 5];

        let a = population.select(Random::from(6..10).selector()).unwrap();
        let b = population.select(Random::from(1..2).selector()).unwrap();
        let c = population
            .as_slice()
            .select(Random::from(1..2).selector())
            .unwrap();

        assert!(a[0] >= 6 && a[0] < 10);
        assert_eq!(b, [1]);
        assert_eq!(c, [1]);
    }
}
