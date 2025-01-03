use super::individual::Individual;

pub trait Fitness: Individual {
    type Value: Ord;

    fn fitness(&self) -> Self::Value;
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
    use super::Fitness;

    #[test]
    fn test_fitness() {
        let a = 10_u8;
        let b = 100_i32;

        assert_eq!(a.fitness(), 10);
        assert_eq!(b.fitness(), 100);
    }
}
