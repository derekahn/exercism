use num_bigint::BigInt;
use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug)]
pub struct Decimal {
    number: BigInt,
    decimal_power: BigInt,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let parts: Vec<&str> = input.split('.').collect();

        Some(Self {
            number: BigInt::parse_bytes(parts.join("").as_bytes(), 10)?,
            decimal_power: BigInt::from(10).pow(parts.get(1).unwrap_or(&"").len() as u32),
        })
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let left: BigInt = self.number.clone() * other.decimal_power.clone();
        let right: BigInt = other.number.clone() * self.decimal_power.clone();

        left == right
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left: BigInt = self.number.clone() * other.decimal_power.clone();
        let right: BigInt = other.number.clone() * self.decimal_power.clone();

        left.partial_cmp(&right)
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let left: BigInt = self.number * rhs.decimal_power.clone();
        let right: BigInt = rhs.number * self.decimal_power.clone();

        Self {
            number: left + right,
            decimal_power: (self.decimal_power * rhs.decimal_power),
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let left: BigInt = self.number * rhs.decimal_power.clone();
        let right: BigInt = rhs.number * self.decimal_power.clone();

        Self {
            number: left - right,
            decimal_power: self.decimal_power * rhs.decimal_power,
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            number: self.number * rhs.number,
            decimal_power: self.decimal_power * rhs.decimal_power,
        }
    }
}
