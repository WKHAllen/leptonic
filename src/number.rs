use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

/// A trait for numeric values.
pub trait Number:
    PartialEq
    + PartialOrd
    + FromStr
    + ToString
    + Default
    + Clone
    + Copy
    + Display
    + Debug
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{
    const NUMBER_MIN: Self;
    const NUMBER_MAX: Self;
    const NUMBER_STEP: Self;
    const DECIMAL: bool;

    fn as_f64(self) -> f64;
}

/// Implements the `Number` trait for integer primitives.
macro_rules! impl_number_int {
    ( $($ty:ty),* ) => {
        $(
            impl Number for $ty {
                const NUMBER_MIN: Self = Self::MIN;
                const NUMBER_MAX: Self = Self::MAX;
                const NUMBER_STEP: Self = 1 as Self;
                const DECIMAL: bool = false;

                fn as_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

/// Implements the `Number` trait for floating point primitives.
macro_rules! impl_number_float {
    ( $($ty:ty),* ) => {
        $(
            impl Number for $ty {
                const NUMBER_MIN: Self = Self::MIN;
                const NUMBER_MAX: Self = Self::MAX;
                const NUMBER_STEP: Self = 1 as Self;
                const DECIMAL: bool = true;

                fn as_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

impl_number_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl_number_float!(f32, f64);
