pub mod nil;

use std::cmp::Reverse;

use num_traits::Zero;
use ordered_float::{FloatCore, OrderedFloat};

pub trait Fitness: Ord {
    fn nil() -> Self;
}

impl<T> Fitness for OrderedFloat<T>
where
    T: FloatCore,
{
    fn nil() -> Self {
        Zero::zero()
    }
}

impl<T> Fitness for Reverse<T>
where
    T: Fitness,
{
    fn nil() -> Self {
        Self(T::nil())
    }
}

macro_rules! impl_fitness_zero {
    ($($type:path),+) => {
        $(impl Fitness for $type {
            fn nil() -> Self {
                Zero::zero()
            }
        })+
    };
}

impl_fitness_zero!(u8, u16, u32, u64, u128, usize);
impl_fitness_zero!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_fitness_default {
    ($($type:path),+) => {
        $(impl Fitness for $type {
            fn nil() -> Self {
                Default::default()
            }
        })+
    };
}

impl_fitness_default!(char, bool);
