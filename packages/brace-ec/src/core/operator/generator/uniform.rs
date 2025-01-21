use std::convert::Infallible;
use std::ops::{Range, RangeInclusive};

use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform as RandUniform};
use rand::distributions::Distribution;

use super::Generator;

pub struct Uniform<T>(RandUniform<T>)
where
    T: SampleUniform;

impl<T> Uniform<T>
where
    T: SampleUniform,
{
    pub fn new<L, H>(low: L, high: H) -> Uniform<T>
    where
        L: SampleBorrow<T> + Sized,
        H: SampleBorrow<T> + Sized,
    {
        Uniform(RandUniform::new(low, high))
    }

    pub fn new_inclusive<L, H>(low: L, high: H) -> Uniform<T>
    where
        L: SampleBorrow<T> + Sized,
        H: SampleBorrow<T> + Sized,
    {
        Uniform(RandUniform::new_inclusive(low, high))
    }
}

impl<T> Generator<T> for Uniform<T>
where
    T: SampleUniform,
{
    type Error = Infallible;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok(self.0.sample(rng))
    }
}

impl<T> From<Range<T>> for Uniform<T>
where
    T: SampleUniform,
{
    fn from(range: Range<T>) -> Uniform<T> {
        Uniform::new(range.start, range.end)
    }
}

impl<T> From<RangeInclusive<T>> for Uniform<T>
where
    T: SampleUniform,
{
    fn from(range: RangeInclusive<T>) -> Uniform<T> {
        Uniform::new_inclusive(range.start(), range.end())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::operator::generator::Generator;

    use super::Uniform;

    #[test]
    fn test_generate() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let a: u8 = Uniform::from(0..10).generate(&mut rng).unwrap();
            let b: usize = Uniform::from(0..10).generate(&mut rng).unwrap();
            let c: i64 = Uniform::from(0..10).generate(&mut rng).unwrap();

            assert!(a < 10);
            assert!(b < 10);
            assert!((0..10).contains(&c));
        }
    }
}
