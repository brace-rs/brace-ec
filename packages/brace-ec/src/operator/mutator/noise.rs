use std::convert::Infallible;
use std::ops::{Add, Range, RangeBounds};

use num_traits::{Bounded, One, SaturatingAdd, SaturatingSub};
use rand::distr::uniform::SampleUniform;

use crate::individual::Individual;
use crate::util::range::get_range;

use super::Mutator;

#[derive(Clone, Debug)]
pub struct Noise<T>(pub Range<T::Genome>)
where
    T: Individual<Genome: Sized>;

impl<T> Noise<T>
where
    T: Individual,
    T::Genome: PartialOrd + Clone + One + Bounded + Add<Output = T::Genome> + Sized,
{
    pub fn new<R>(range: R) -> Self
    where
        R: RangeBounds<T::Genome>,
    {
        Self(get_range(range))
    }
}

impl<T> Mutator<T> for Noise<T>
where
    T: Individual,
    T::Genome: Clone + PartialOrd + SaturatingAdd + SaturatingSub + SampleUniform,
{
    type Error = Infallible;

    fn mutate<Rng>(&self, mut individual: T, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        let genome = match rng.random_bool(0.5) {
            true => individual
                .genome()
                .saturating_add(&rng.random_range(self.0.clone())),
            false => individual
                .genome()
                .saturating_sub(&rng.random_range(self.0.clone())),
        };

        *individual.genome_mut() = genome;

        Ok(individual)
    }
}

impl<T> Default for Noise<T>
where
    T: Individual,
    T::Genome: PartialOrd + Clone + One + Bounded + Add<Output = T::Genome> + Sized,
{
    fn default() -> Self {
        Self::new(T::Genome::one()..=T::Genome::one())
    }
}

#[cfg(test)]
mod tests {
    use crate::individual::Individual;

    use super::Noise;

    #[test]
    fn test_mutate() {
        for _ in 0..1_000 {
            let a = 150.mutated(Noise(1..11)).unwrap();

            assert!(a != 150);
            assert!(a <= 160);
            assert!(a >= 140);

            let b = 250.mutated(Noise::new(1..=10)).unwrap();

            assert!(b != 250);
            assert!(b <= 260);
            assert!(b >= 240);
        }

        for _ in 0..10 {
            let c = 350.mutated(Noise::default()).unwrap();

            assert!(c == 349 || c == 351);
        }
    }
}
