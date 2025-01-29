use ordered_float::OrderedFloat;

use super::individual::Individual;

pub trait Fitness: Individual {
    type Fitness: Ord;

    fn fitness(&self) -> &Self::Fitness;
}

impl Fitness for f32 {
    type Fitness = OrderedFloat<Self>;

    fn fitness(&self) -> &Self::Fitness {
        self.into()
    }
}

impl Fitness for f64 {
    type Fitness = OrderedFloat<Self>;

    fn fitness(&self) -> &Self::Fitness {
        self.into()
    }
}

macro_rules! impl_fitness {
    ($($type:path),+) => {
        $(impl Fitness for $type {
            type Fitness = Self;

            fn fitness(&self) -> &Self::Fitness {
                self
            }
        })+
    };
}

impl_fitness!(u8, u16, u32, u64, u128, usize);
impl_fitness!(i8, i16, i32, i64, i128, isize);

pub trait FitnessMut: Fitness {
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
}

#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;

    use super::Fitness;

    #[test]
    fn test_fitness() {
        let a = 10_u8;
        let b = 100_i32;
        let c = 1.5;

        assert_eq!(*a.fitness(), 10);
        assert_eq!(*b.fitness(), 100);
        assert_eq!(*c.fitness(), OrderedFloat(1.5));
    }
}
