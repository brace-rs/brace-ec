use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::{Range, RangeInclusive};

use rand::distributions::uniform::SampleUniform;
use rand::distributions::{
    Alphanumeric, Bernoulli, BernoulliError, Distribution, Standard, Uniform,
};

use super::Generator;

pub struct Random<T, D, E = Infallible>
where
    D: Distribution<T>,
{
    distribution: Result<D, E>,
    marker: PhantomData<fn() -> T>,
}

impl<T, D> Random<T, D>
where
    D: Distribution<T>,
{
    pub fn new(distribution: D) -> Self {
        Self {
            distribution: Ok(distribution),
            marker: PhantomData,
        }
    }
}

impl<T> Random<T, Standard>
where
    Standard: Distribution<T>,
{
    pub fn standard() -> Self {
        Self::new(Standard)
    }
}

impl<T> Random<T, Alphanumeric>
where
    Alphanumeric: Distribution<T>,
{
    pub fn alphanumeric() -> Self {
        Self::new(Alphanumeric)
    }
}

impl<T> Random<T, Uniform<T>>
where
    T: SampleUniform,
    Uniform<T>: Distribution<T>,
{
    pub fn uniform(range: Range<T>) -> Self {
        Self::new(Uniform::from(range))
    }

    pub fn uniform_inclusive(range: RangeInclusive<T>) -> Self {
        Self::new(Uniform::from(range))
    }
}

impl<T> Random<T, Bernoulli, BernoulliError>
where
    Bernoulli: Distribution<T>,
{
    pub fn bernoulli(probability: f64) -> Self {
        Self {
            distribution: Bernoulli::new(probability),
            marker: PhantomData,
        }
    }
}

impl<T, D, E> Generator<T> for Random<T, D, E>
where
    D: Distribution<T>,
    E: Clone,
{
    type Error = E;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.distribution
            .as_ref()
            .map(|distribution| distribution.sample(rng))
            .map_err(Clone::clone)
    }
}

impl<T> From<Range<T>> for Random<T, Uniform<T>>
where
    T: SampleUniform,
    Uniform<T>: Distribution<T>,
{
    fn from(range: Range<T>) -> Self {
        Self::uniform(range)
    }
}

impl<T> From<RangeInclusive<T>> for Random<T, Uniform<T>>
where
    T: SampleUniform,
    Uniform<T>: Distribution<T>,
{
    fn from(range: RangeInclusive<T>) -> Self {
        Self::uniform_inclusive(range)
    }
}

#[cfg(test)]
mod tests {
    use rand::distributions::BernoulliError;

    use crate::core::operator::generator::Generator;

    use super::Random;

    #[test]
    fn test_generate() {
        let mut rng = rand::thread_rng();

        let _: u8 = Random::standard().generate(&mut rng).unwrap();
        let _: u8 = Random::alphanumeric().generate(&mut rng).unwrap();
        let _: bool = Random::bernoulli(0.5).generate(&mut rng).unwrap();

        let a = Random::bernoulli(0.0).generate(&mut rng);
        let b = Random::bernoulli(1.0).generate(&mut rng);
        let c = Random::bernoulli(100.0).generate(&mut rng);

        assert_eq!(a, Ok(false));
        assert_eq!(b, Ok(true));
        assert_eq!(c, Err(BernoulliError::InvalidProbability));

        for _ in 0..100 {
            let d: u8 = Random::from(0..10).generate(&mut rng).unwrap();
            let e: usize = Random::from(0..10).generate(&mut rng).unwrap();
            let f: i64 = Random::from(0..10).generate(&mut rng).unwrap();

            assert!(d < 10);
            assert!(e < 10);
            assert!((0..10).contains(&f));
        }
    }
}
