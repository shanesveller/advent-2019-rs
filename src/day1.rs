use core::option::Option;
use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Fuel(pub usize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Mass(pub usize);

impl Fuel {
    /// # Examples
    /// ```rust
    /// use advent_of_code_2019::day1::{Fuel, Mass};
    ///
    /// assert_eq!(Fuel(4).mass(), Mass(4));
    /// assert_eq!(Fuel(8).mass(), Mass(8));
    /// ````
    pub fn mass(self) -> Mass {
        Mass { 0: self.0 }
    }

    /// # Examples
    /// ```rust
    /// use advent_of_code_2019::day1::{Fuel, Mass};
    ///
    /// assert_eq!(Fuel(2).required_fuel(), Fuel(0));
    /// assert_eq!(Fuel(654).required_fuel(), Fuel(312));
    /// ````
    pub fn required_fuel(self) -> Self {
        self.fold(Self(0), |acc, f| acc + f)
    }
}

impl Add for Fuel {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
        }
    }
}

impl AddAssign for Fuel {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            0: self.0 + other.0,
        }
    }
}

impl fmt::Display for Fuel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fuel: {} units", self.0)
    }
}

impl Iterator for Fuel {
    type Item = Self;

    /// # Examples
    /// ```rust
    /// use advent_of_code_2019::day1::{Fuel, Mass};
    ///
    /// assert_eq!(Fuel(2).next(), None);
    /// assert_eq!(Fuel(3).next(), None);
    /// assert_eq!(Fuel(5).next(), None);
    /// assert_eq!(Fuel(21).next(), Some(Fuel(5)));
    /// assert_eq!(Fuel(70).next(), Some(Fuel(21)));
    /// assert_eq!(Fuel(216).next(), Some(Fuel(70)));
    /// assert_eq!(Fuel(654).next(), Some(Fuel(216)));
    /// ````
    fn next(&mut self) -> Option<Self::Item> {
        let next: Self = self.mass().required_fuel();
        *self = next;

        if next > Self(0) {
            Some(next)
        } else {
            None
        }
    }
}

impl Mass {
    const DIVIDE_BY: f32 = 3.0;
    const REDUCE_BY: f32 = 2.0;

    /// # Examples
    /// ```rust
    /// use advent_of_code_2019::day1::{Fuel, Mass};
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

    fn all_required_fuel(self) -> Fuel {
        let f = self.required_fuel();
        f + f.required_fuel()
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Mass> {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .map(Mass)
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[Mass]) -> Fuel {
    input
        .iter()
        .map(|m| m.required_fuel())
        .fold(Fuel(0), |acc, f| acc + f)
}

#[aoc(day1, part2)]
fn solve_part2(input: &[Mass]) -> Fuel {
    input
        .iter()
        .fold(Fuel(0), |acc, m| acc + m.all_required_fuel())
}
