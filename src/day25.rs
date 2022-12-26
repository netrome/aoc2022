pub fn p1(input: &str) -> String {
    Snafu::from(parse_input(input).iter().map(i64::from).sum::<i64>()).to_string()
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn parse_input(input: &str) -> Vec<Snafu> {
    input
        .lines()
        .map(|line| line.parse().expect("SNAFU parsing failed"))
        .collect()
}

#[derive(Debug)]
struct Snafu(Vec<SDigit>);

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.0.iter() {
            char::from(*digit).fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum SDigit {
    P2,
    P1,
    Z,
    M1,
    M2,
}

impl SDigit {
    fn value(&self) -> i64 {
        match self {
            Self::P2 => 2,
            Self::P1 => 1,
            Self::Z => 0,
            Self::M1 => -1,
            Self::M2 => -2,
        }
    }
}

impl From<i64> for Snafu {
    fn from(value: i64) -> Self {
        let max_pow = ilog5(value.abs());
        let mut digits = Vec::new();
        from_i64(max_pow, value, &mut digits);

        // Trim leading zeros
        while let Some(SDigit::Z) = digits.first() {
            digits.remove(0);
        }

        Self(digits)
    }
}

fn from_i64(pow: u32, value: i64, digits: &mut Vec<SDigit>) {
    let mut min_remainder = i64::MAX;
    let mut min_digit = SDigit::Z;

    for digit in -2..=2 {
        let cmp_value = digit * 5i64.pow(pow);
        let rem = value - cmp_value;

        if rem.abs() < min_remainder.abs() {
            min_remainder = rem;
            min_digit = digit.try_into().unwrap();
        }
    }

    digits.push(min_digit);

    if let Some(next) = pow.checked_sub(1) {
        from_i64(next, min_remainder, digits)
    }
}

// Ceil of log_5(val)
fn ilog5(val: i64) -> u32 {
    let mut pow = 0;

    while val > 5i64.pow(pow) {
        pow += 1
    }

    pow
}

impl From<&Snafu> for i64 {
    fn from(value: &Snafu) -> Self {
        value
            .0
            .iter()
            .rev()
            .enumerate()
            .map(|(pow, digit)| 5i64.pow(pow as u32) * digit.value())
            .sum()
    }
}

impl TryFrom<i64> for SDigit {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            2 => Self::P2,
            1 => Self::P1,
            0 => Self::Z,
            -1 => Self::M1,
            -2 => Self::M2,
            other => anyhow::bail!("Invalid SNAFU digit: {}", other),
        })
    }
}

impl TryFrom<char> for SDigit {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Self::P2,
            '1' => Self::P1,
            '0' => Self::Z,
            '-' => Self::M1,
            '=' => Self::M2,
            other => anyhow::bail!("Invalid SNAFU digit: {}", other),
        })
    }
}

impl From<SDigit> for char {
    fn from(d: SDigit) -> Self {
        match d {
            SDigit::P2 => '2',
            SDigit::P1 => '1',
            SDigit::Z => '0',
            SDigit::M1 => '-',
            SDigit::M2 => '=',
        }
    }
}

impl FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .trim()
            .chars()
            .map(|c| c.try_into())
            .collect::<anyhow::Result<Vec<SDigit>>>()?;

        Ok(Self(digits))
    }
}

use std::{convert::TryFrom, fmt::Display, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(25, 1, p1));
inventory::submit!(Solution::new(25, 2, p2));
