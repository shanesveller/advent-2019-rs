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
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::copy_iterator
)]

use core::convert::From;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opcode {
    Add,
    Multiply,
    Halt,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseOpcodeError;

impl From<usize> for Opcode {
    /// # Examples
    /// ```rust
    /// use day_2::Opcode;
    /// assert_eq!(Opcode::from(1), Opcode::Add);
    /// assert_eq!(Opcode::from(2), Opcode::Multiply);
    /// assert_eq!(Opcode::from(3), Opcode::Unknown);
    /// assert_eq!(Opcode::from(99), Opcode::Halt);
    /// ````
    fn from(n: usize) -> Self {
        match n {
            1 => Self::Add,
            2 => Self::Multiply,
            99 => Self::Halt,
            _ => Self::Unknown,
        }
    }
}

/// ```rust
/// let str = "1,2,3\n4,5";
/// assert_eq!(day_2::parse_input(str), [1,2,3,4,5]);
/// ```
pub fn parse_input(s: &str) -> Vec<usize> {
    s.lines()
        .flat_map(|l| l.split(','))
        .map(|s| s.parse().unwrap())
        .collect()
}
