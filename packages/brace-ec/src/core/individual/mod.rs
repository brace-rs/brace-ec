pub mod reversed;
pub mod scored;

use ordered_float::OrderedFloat;

use self::reversed::Reversed;
use self::scored::Scored;

use super::fitness::nil::Nil;
use super::fitness::Fitness;
use super::operator::mutator::Mutator;

pub trait Individual {
    type Genome: ?Sized;
    type Fitness: Fitness;

    fn genome(&self) -> &Self::Genome;

    fn genome_mut(&mut self) -> &mut Self::Genome;

    fn fitness(&self) -> &Self::Fitness;

    fn fitness_mut(&mut self) -> &mut Self::Fitness;

    fn set_fitness(&mut self, fitness: Self::Fitness) {
        *self.fitness_mut() = fitness;
    }

    fn with_fitness(mut self, fitness: Self::Fitness) -> Self
    where
        Self: Sized,
    {
        self.set_fitness(fitness);
        self
    }

    fn mutate<M>(self, mutator: M) -> Result<Self, M::Error>
    where
        M: Mutator<Self>,
        Self: Sized,
    {
        mutator.mutate(self, &mut rand::rng())
    }

    fn scored<S>(self) -> Scored<Self, S>
    where
        S: Fitness,
        Self: Sized,
    {
        Scored::from(self)
    }

    fn reversed(self) -> Reversed<Self>
    where
        Self: Sized,
    {
        Reversed::new(self)
    }
}

impl<T, const N: usize> Individual for [T; N] {
    type Genome = [T];
    type Fitness = Nil;

    fn genome(&self) -> &Self::Genome {
        self
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self
    }

    fn fitness(&self) -> &Self::Fitness {
        Nil::r#ref()
    }

    fn fitness_mut(&mut self) -> &mut Self::Fitness {
        Nil::r#mut()
    }
}

impl<T> Individual for Vec<T> {
    type Genome = [T];
    type Fitness = Nil;

    fn genome(&self) -> &Self::Genome {
        self
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self
    }

    fn fitness(&self) -> &Self::Fitness {
        Nil::r#ref()
    }

    fn fitness_mut(&mut self) -> &mut Self::Fitness {
        Nil::r#mut()
    }
}

impl Individual for f32 {
    type Genome = Self;
    type Fitness = OrderedFloat<Self>;

    fn genome(&self) -> &Self::Genome {
        self
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self
    }

    fn fitness(&self) -> &Self::Fitness {
        self.into()
    }

    fn fitness_mut(&mut self) -> &mut Self::Fitness {
        self.into()
    }
}

impl Individual for f64 {
    type Genome = Self;
    type Fitness = OrderedFloat<Self>;

    fn genome(&self) -> &Self::Genome {
        self
    }

    fn genome_mut(&mut self) -> &mut Self::Genome {
        self
    }

    fn fitness(&self) -> &Self::Fitness {
        self.into()
    }

    fn fitness_mut(&mut self) -> &mut Self::Fitness {
        self.into()
    }
}

macro_rules! impl_individual {
    ($($type:path),+) => {
        $(impl Individual for $type {
            type Genome = Self;
            type Fitness = Self;

            fn genome(&self) -> &Self::Genome {
                self
            }

            fn genome_mut(&mut self) -> &mut Self::Genome {
                self
            }

            fn fitness(&self) -> &Self::Fitness {
                self
            }

            fn fitness_mut(&mut self) -> &mut Self::Fitness {
                self
            }
        })+
    };
}

impl_individual!(u8, u16, u32, u64, u128, usize);
impl_individual!(i8, i16, i32, i64, i128, isize);
impl_individual!(char, bool);

#[cfg(test)]
mod tests {
    use crate::core::fitness::nil::Nil;

    use super::Individual;

    fn erase<G: ?Sized, I: Individual<Genome = G>>(
        individual: I,
    ) -> impl Individual<Genome = G, Fitness = I::Fitness> {
        individual
    }

    #[test]
    fn test_individual_array() {
        let mut individual = erase([0, 0]);

        assert_eq!(individual.genome(), [0, 0]);
        assert_eq!(individual.fitness(), &Nil::new());
        assert_eq!(individual.fitness_mut(), &Nil::new());
    }

    #[test]
    fn test_individual_vec() {
        let mut individual = erase(vec![0, 0]);

        assert_eq!(individual.genome(), [0, 0]);
        assert_eq!(individual.fitness(), &Nil::new());
        assert_eq!(individual.fitness_mut(), &Nil::new());
    }
}
