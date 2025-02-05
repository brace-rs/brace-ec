use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::{Range, RangeInclusive};

use rand::distr::uniform::{Error as UniformError, SampleUniform};
use rand::distr::{
    Alphanumeric, Bernoulli, BernoulliError, Distribution, StandardUniform, Uniform,
};

use crate::individual::Individual;

use super::Generator;

pub struct Random<T, D, E = Infallible>
where
    T: Individual<Genome: Sized> + From<T::Genome>,
    D: Distribution<T::Genome>,
{
    distribution: Result<D, E>,
    marker: PhantomData<fn() -> T>,
}

impl<T, D> Random<T, D>
where
    T: Individual<Genome: Sized> + From<T::Genome>,
    D: Distribution<T::Genome>,
{
    pub fn new(distribution: D) -> Self {
        Self {
            distribution: Ok(distribution),
            marker: PhantomData,
        }
    }
}

impl<T> Random<T, StandardUniform>
where
    T: Individual<Genome: Sized> + From<T::Genome>,
    StandardUniform: Distribution<T::Genome>,
{
    pub fn standard() -> Self {
        Self::new(StandardUniform)
    }
}

impl<T> Random<T, Alphanumeric>
where
    T: Individual<Genome: Sized> + From<T::Genome>,
    Alphanumeric: Distribution<T::Genome>,
{
    pub fn alphanumeric() -> Self {
        Self::new(Alphanumeric)
    }
}

impl<T> Random<T, Uniform<T::Genome>, UniformError>
where
    T: Individual<Genome: SampleUniform> + From<T::Genome>,
    Uniform<T::Genome>: Distribution<T::Genome>,
{
    pub fn uniform(range: Range<T::Genome>) -> Self {
        Self {
            distribution: Uniform::try_from(range),
            marker: PhantomData,
        }
    }

    pub fn uniform_inclusive(range: RangeInclusive<T::Genome>) -> Self {
        Self {
            distribution: Uniform::try_from(range),
            marker: PhantomData,
        }
    }
}

impl<T> Random<T, Bernoulli, BernoulliError>
where
    T: Individual<Genome: Sized> + From<T::Genome>,
    Bernoulli: Distribution<T::Genome>,
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
    T: Individual<Genome: Sized> + From<T::Genome>,
    D: Distribution<T::Genome>,
    E: Clone,
{
    type Error = E;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        self.distribution
            .as_ref()
            .map(|distribution| distribution.sample(rng).into())
            .map_err(Clone::clone)
    }
}

impl<T> From<Range<T::Genome>> for Random<T, Uniform<T::Genome>, UniformError>
where
    T: Individual<Genome: SampleUniform> + From<T::Genome>,
    Uniform<T::Genome>: Distribution<T::Genome>,
{
    fn from(range: Range<T::Genome>) -> Self {
        Self::uniform(range)
    }
}

impl<T> From<RangeInclusive<T::Genome>> for Random<T, Uniform<T::Genome>, UniformError>
where
    T: Individual<Genome: SampleUniform> + From<T::Genome>,
    Uniform<T::Genome>: Distribution<T::Genome>,
{
    fn from(range: RangeInclusive<T::Genome>) -> Self {
        Self::uniform_inclusive(range)
    }
}

#[cfg(test)]
mod tests {
    use rand::distr::BernoulliError;

    use crate::individual::scored::Scored;
    use crate::operator::generator::Generator;

    use super::Random;

    #[test]
    fn test_generate() {
        let mut rng = rand::rng();

        let _: u8 = Random::standard().generate(&mut rng).unwrap();
        let _: u8 = Random::alphanumeric().generate(&mut rng).unwrap();
        let _: bool = Random::bernoulli(0.5).generate(&mut rng).unwrap();

        let _: Scored<u8, u8> = Random::standard().generate(&mut rng).unwrap();
        let _: Scored<u8, usize> = Random::alphanumeric().generate(&mut rng).unwrap();
        let _: Scored<bool, i32> = Random::bernoulli(0.5).generate(&mut rng).unwrap();

        let a = Random::bernoulli(0.0).generate(&mut rng);
        let b = Random::bernoulli(1.0).generate(&mut rng);
        let c = Random::<bool, _, _>::bernoulli(100.0).generate(&mut rng);

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
