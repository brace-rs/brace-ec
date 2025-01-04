use std::convert::Infallible;
use std::ops::{Add, Range, RangeBounds};

use num_traits::{Bounded, One, SaturatingAdd, SaturatingSub};
use rand::distributions::uniform::SampleUniform;
use rand::Rng;

use crate::core::individual::Individual;
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

impl<T> Mutator for Noise<T>
where
    T: Individual,
    T::Genome: Clone + PartialOrd + SaturatingAdd + SaturatingSub + SampleUniform,
{
    type Individual = T;
    type Error = Infallible;

    fn mutate<R>(
        &self,
        mut individual: Self::Individual,
        rng: &mut R,
    ) -> Result<Self::Individual, Self::Error>
    where
        R: Rng + ?Sized,
    {
        let genome = match rng.gen_bool(0.5) {
            true => individual
                .genome()
                .saturating_add(&rng.gen_range(self.0.clone())),
            false => individual
                .genome()
                .saturating_sub(&rng.gen_range(self.0.clone())),
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
    use crate::core::individual::Individual;

    use super::Noise;

    #[test]
    fn test_mutate() {
        for _ in 0..1_000 {
            let a = 150.mutate(Noise(1..11)).unwrap();

            assert!(a != 150);
            assert!(a <= 160);
            assert!(a >= 140);

            let b = 250.mutate(Noise::new(1..=10)).unwrap();

            assert!(b != 250);
            assert!(b <= 260);
            assert!(b >= 240);
        }

        for _ in 0..10 {
            let c = 350.mutate(Noise::default()).unwrap();

            assert!(c == 349 || c == 351);
        }
    }
}
