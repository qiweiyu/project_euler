extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::{Add, Mul, Div};

#[derive(Clone)]
pub struct FractionBig {
    numerator: BigUint,
    denominator: BigUint,
}

impl FractionBig {
    pub fn new(numerator: BigUint, denominator: BigUint) -> Self {
        assert_ne!(denominator, Zero::zero());
        FractionBig {
            numerator,
            denominator,
        }
    }

    #[inline]
    pub fn simplify(&mut self) {
        let gcd = _gcd(self.numerator.clone(), self.denominator.clone());
        self.numerator = self.numerator.clone() / gcd.clone();
        self.denominator = self.denominator.clone() / gcd.clone();
    }

    pub fn expand(&mut self, size: &BigUint) {
        assert_ne!(*size, Zero::zero());
        self.numerator = self.numerator.clone() * size;
        self.denominator = self.denominator.clone() * size;
    }

    pub fn to_tuple(&self) -> (BigUint, BigUint) {
        (self.numerator.clone(), self.denominator.clone())
    }

    pub fn to_tuple_string(&self) -> (String, String) {
        (self.numerator.to_str_radix(10), self.denominator.to_str_radix(10))
    }
}

impl Add<u64> for FractionBig {
    type Output = FractionBig;

    fn add(mut self, other: u64) -> Self {
        self.numerator = self.denominator.clone() * other + self.numerator.clone();
        self.simplify();
        self
    }
}

impl Add<FractionBig> for FractionBig {
    type Output = FractionBig;

    fn add(mut self, mut other: Self) -> Self {
        let old_de = self.denominator.clone();
        self.expand(&other.denominator);
        other.expand(&old_de);
        let mut res = Self {
            numerator: self.numerator + other.numerator,
            denominator: self.denominator,
        };
        res.simplify();
        res
    }
}

impl Add<FractionBig> for u64 {
    type Output = FractionBig;

    fn add(self, mut other: FractionBig) -> FractionBig {
        other.numerator = other.denominator.clone() * self + other.numerator.clone();
        other.simplify();
        other
    }
}

impl Mul<u64> for FractionBig {
    type Output = FractionBig;

    fn mul(mut self, other: u64) -> Self {
        self.numerator = self.numerator.clone() * other;
        self.simplify();
        self
    }
}

impl Mul<FractionBig> for FractionBig {
    type Output = FractionBig;

    fn mul(mut self, mut other: Self) -> Self {
        let mut res = Self {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        };
        res.simplify();
        res
    }
}

impl Mul<FractionBig> for u64 {
    type Output = FractionBig;

    fn mul(self, mut other: FractionBig) -> FractionBig {
        other.numerator = other.numerator.clone() * self;
        other.simplify();
        other
    }
}

impl Div<u64> for FractionBig {
    type Output = FractionBig;

    fn div(mut self, rhs: u64) -> Self {
        self.denominator = self.denominator.clone() * rhs;
        self.simplify();
        self
    }
}

impl Div<FractionBig> for FractionBig {
    type Output = FractionBig;

    fn div(mut self, mut rhs: Self) -> Self {
        let mut res = Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        };
        res.simplify();
        res
    }
}

impl Div<FractionBig> for u64 {
    type Output = FractionBig;

    fn div(self, rhs: FractionBig) -> FractionBig {
        let mut res = FractionBig {
            numerator: rhs.denominator,
            denominator: rhs.numerator,
        };
        res * self
    }
}

impl PartialEq for FractionBig {
    fn eq(&self, other: &Self) -> bool {
        &self.numerator * &other.denominator == &other.numerator * &self.denominator
    }
}

impl Eq for FractionBig {}

use std::fmt;

impl fmt::Debug for FractionBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator.to_str_radix(10), self.denominator.to_str_radix(10))
    }
}

impl From<u64> for FractionBig {
    fn from(n: u64) -> FractionBig {
        FractionBig {
            numerator: BigUint::from(n),
            denominator: One::one(),
        }
    }
}

#[inline]
fn _gcd(a: BigUint, b: BigUint) -> BigUint {
    if b == Zero::zero() {
        a
    } else {
        _gcd(b.clone(), a % b)
    }
}

#[inline]
fn _lcm(a: BigUint, b: BigUint) -> BigUint {
    let gcd = _gcd(a.clone(), b.clone());
    (a / gcd) * b
}