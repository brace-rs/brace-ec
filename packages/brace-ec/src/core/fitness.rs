use ordered_float::OrderedFloat;

use super::individual::Individual;

pub trait Fitness: Individual {
    type Value: Ord;

    fn fitness(&self) -> &Self::Value;
}

impl Fitness for f32 {
    type Value = OrderedFloat<Self>;

    fn fitness(&self) -> &Self::Value {
        self.into()
    }
}

impl Fitness for f64 {
    type Value = OrderedFloat<Self>;

    fn fitness(&self) -> &Self::Value {
        self.into()
    }
}

macro_rules! impl_fitness {
    ($($type:path),+) => {
        $(impl Fitness for $type {
            type Value = Self;

            fn fitness(&self) -> &Self::Value {
                self
            }
        })+
    };
}

impl_fitness!(u8, u16, u32, u64, u128, usize);
impl_fitness!(i8, i16, i32, i64, i128, isize);

pub trait FitnessMut: Fitness {
    fn fitness_mut(&mut self) -> &mut Self::Value;

    fn set_fitness(&mut self, fitness: Self::Value) {
        *self.fitness_mut() = fitness;
    }

    fn with_fitness(mut self, fitness: Self::Value) -> Self
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
