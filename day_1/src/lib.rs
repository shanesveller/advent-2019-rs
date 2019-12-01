#![forbid(unsafe_code)]
#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    clippy::all,
    clippy::pedantic
)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Fuel(pub usize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Mass(pub usize);

impl Add for Fuel {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
        }
    }
}

impl Mass {
    const DIVIDE_BY: f32 = 3.0;
    const REDUCE_BY: f32 = 2.0;

    /// # Examples
    /// ```rust
    /// use day_1::{Fuel, Mass};
    ///
    /// assert_eq!(Mass(2).required_fuel(), Fuel(0));
    /// assert_eq!(Mass(12).required_fuel(), Fuel(2));
    /// assert_eq!(Mass(14).required_fuel(), Fuel(2));
    /// assert_eq!(Mass(1969).required_fuel(), Fuel(654));
    /// assert_eq!(Mass(100756).required_fuel(), Fuel(33583));
    /// ````
    pub fn required_fuel(self) -> Fuel {
        let Self(n) = self;

        let mut n: f32 = (n as f32 / Self::DIVIDE_BY).trunc();
        n -= Self::REDUCE_BY;

        if n >= 0.0 {
            Fuel(n as usize)
        } else {
            Fuel(0)
        }
    }
}
