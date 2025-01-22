use std::marker::PhantomData;

use crate::core::operator::generator::Generator;
use crate::core::population::Population;

use super::Selector;

pub struct Generate<T, P> {
    generator: T,
    marker: PhantomData<fn() -> P>,
}

impl<T, P> Generate<T, P> {
    pub fn new(generator: T) -> Self {
        Self {
            generator,
            marker: PhantomData,
        }
    }
}

impl<P, T> Selector<P> for Generate<T, P>
where
    P: Population,
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
    use crate::core::operator::generator::uniform::Uniform;
    use crate::core::operator::generator::Generator;
    use crate::core::population::Population;

    #[test]
    fn test_select() {
        let population = [1, 2, 3, 4, 5];

        let a = population.select(Uniform::from(6..10).selector()).unwrap();
        let b = population.select(Uniform::from(1..2).selector()).unwrap();

        assert!(a[0] >= 6 && a[0] < 10);
        assert_eq!(b, [1]);
    }
}
