pub mod reversed;
pub mod scored;

use rand::thread_rng;

use self::reversed::Reversed;
use self::scored::Scored;

use super::fitness::Fitness;
use super::operator::mutator::Mutator;

pub trait Individual {
    type Genome: ?Sized;

    fn genome(&self) -> &Self::Genome;

    fn genome_mut(&mut self) -> &mut Self::Genome;

    fn mutate<M>(self, mutator: M) -> Result<Self, M::Error>
    where
        M: Mutator<Self>,
        Self: Sized,
    {
        mutator.mutate(self, &mut thread_rng())
    }

    fn scored<S>(self) -> Scored<Self, S>
    where
        S: Default,
        Self: Sized,
    {
        Scored::from(self)
    }

    fn reversed(self) -> Reversed<Self>
    where
        Self: Fitness + Sized,
    {
        Reversed::new(self)
    }
}

impl<T, const N: usize> Individual for [T; N] {
    type Genome = [T];

    fn genome(&self) -> &Self::Genome {
        self
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self
    }
}

impl<T> Individual for Vec<T> {
    type Genome = [T];

    fn genome(&self) -> &Self::Genome {
        self
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self
    }
}

macro_rules! impl_individual {
    ($($type:path),+) => {
        $(impl Individual for $type {
            type Genome = Self;

            fn genome(&self) -> &Self::Genome {
                self
            }

            fn genome_mut(&mut self) -> &mut Self::Genome {
                self
            }
        })+
    };
}

impl_individual!(u8, u16, u32, u64, u128, usize);
impl_individual!(i8, i16, i32, i64, i128, isize);
impl_individual!(f32, f64, char, bool);

#[cfg(test)]
mod tests {
    use super::Individual;

    fn erase<G: ?Sized, I: Individual<Genome = G>>(individual: I) -> impl Individual<Genome = G> {
        individual
    }

    #[test]
    fn test_individual_array() {
        let individual = erase([0, 0]);

        assert_eq!(individual.genome(), [0, 0]);
    }

    #[test]
    fn test_individual_vec() {
        let individual = erase(vec![0, 0]);

        assert_eq!(individual.genome(), [0, 0]);
    }
}
