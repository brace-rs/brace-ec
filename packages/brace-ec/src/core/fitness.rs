use std::cmp::Reverse;

use super::individual::Individual;

pub trait Fitness: Individual {
    type Value: Ord;

    fn fitness(&self) -> Self::Value;
}

impl<T> Fitness for Reverse<T>
where
    T: Fitness,
{
    type Value = Reverse<T::Value>;

    fn fitness(&self) -> Self::Value {
        Reverse(self.0.fitness())
    }
}

macro_rules! impl_fitness {
    ($($type:path),+) => {
        $(impl Fitness for $type {
            type Value = Self;

            fn fitness(&self) -> Self::Value {
                *self
            }
        })+
    };
}

impl_fitness!(u8, u16, u32, u64, u128, usize);
impl_fitness!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use super::Fitness;

    #[test]
    fn test_fitness() {
        let a = 10_u8;
        let b = 100_i32;
        let c = Reverse(50_u64);

        assert_eq!(a.fitness(), 10);
        assert_eq!(b.fitness(), 100);
        assert_eq!(c.fitness(), Reverse(50));
    }
}
