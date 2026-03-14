use std::convert::Infallible;
use std::fmt::{self, Display};

pub trait TrySum<T = Self, S = Standard>: Sized {
    type Error;

    fn try_sum<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: Iterator<Item = T>;
}

macro_rules! impl_try_sum {
    (* -> $($type:path),+) => {
        impl_try_sum!(Standard -> $($type),+);
        impl_try_sum!(Saturate -> $($type),+);
        impl_try_sum!(Wrap -> $($type),+);
        impl_try_sum!(Strict -> $($type),+);
        impl_try_sum!(Check -> $($type),+);
    };
    (Standard -> $($type:path),+) => {
        $(
            impl<T> TrySum<T, Standard> for $type
            where
                T: Into<Self>,
            {
                type Error = Infallible;

                fn try_sum<I>(iter: I) -> Result<Self, Self::Error>
                where
                    I: Iterator<Item = T>,
                {
                    Ok(iter.map(Into::into).sum())
                }
            }
        )+
    };
    (Saturate -> $($type:path),+) => {
        $(
            impl<T> TrySum<T, Saturate> for $type
            where
                T: Into<Self>,
            {
                type Error = Infallible;

                fn try_sum<I>(iter: I) -> Result<Self, Self::Error>
                where
                    I: Iterator<Item = T>,
                {
                    Ok(iter.fold(Self::default(), |total, item| {
                        Self::saturating_add(total, item.into())
                    }))
                }
            }
        )+
    };
    (Wrap -> $($type:path),+) => {
        $(
            impl<T> TrySum<T, Wrap> for $type
            where
                T: Into<Self>,
            {
                type Error = Infallible;

                fn try_sum<I>(iter: I) -> Result<Self, Self::Error>
                where
                    I: Iterator<Item = T>,
                {
                    Ok(iter.fold(Self::default(), |total, item| {
                        Self::wrapping_add(total, item.into())
                    }))
                }
            }
        )+
    };
    (Strict -> $($type:path),+) => {
        $(
            impl<T> TrySum<T, Strict> for $type
            where
                T: Into<Self>,
            {
                type Error = Infallible;

                fn try_sum<I>(iter: I) -> Result<Self, Self::Error>
                where
                    I: Iterator<Item = T>,
                {
                    Ok(iter.fold(Self::default(), |total, item| {
                        Self::strict_add(total, item.into())
                    }))
                }
            }
        )+
    };
    (Check -> $($type:path),+) => {
        $(
            impl<T> TrySum<T, Check> for $type
            where
                T: Into<Self>,
            {
                type Error = Overflow;

                fn try_sum<I>(mut iter: I) -> Result<Self, Self::Error>
                where
                    I: Iterator<Item = T>,
                {
                    iter.try_fold(Self::default(), |total, item| {
                        Self::checked_add(total, item.into())
                    })
                    .ok_or(Overflow)
                }
            }
        )+
    }
}

impl_try_sum!(* -> usize, u8, u16, u32, u64, u128);
impl_try_sum!(* -> isize, i8, i16, i32, i64, i128);
impl_try_sum!(Standard -> f32, f64);

#[derive(Clone, Copy)]
pub struct Standard;

#[derive(Clone, Copy)]
pub struct Saturate;

#[derive(Clone, Copy)]
pub struct Wrap;

#[derive(Clone, Copy)]
pub struct Strict;

#[derive(Clone, Copy)]
pub struct Check;

#[derive(Debug)]
pub struct Overflow;

impl Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "overflow")
    }
}

impl std::error::Error for Overflow {}
