use std::ops::{Add, Mul, Div};

#[derive(Copy, Clone)]
pub struct Fraction {
    numerator: u128,
    denominator: u128,
}

impl Fraction {
    pub fn new(numerator: u128, denominator: u128) -> Self {
        assert_ne!(denominator, 0);
        Fraction {
            numerator,
            denominator,
        }
    }

    #[inline]
    pub fn simplify(&mut self) {
        let gcd = _gcd(self.numerator, self.denominator);
        self.numerator = self.numerator / gcd;
        self.denominator = self.denominator / gcd;
    }

    pub fn to_f64(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    pub fn to_tuple(self) -> (u128, u128) {
        (self.numerator, self.denominator)
    }
}

impl Add<u128> for Fraction {
    type Output = Fraction;

    fn add(mut self, other: u128) -> Self {
        self.simplify();
        self.numerator = self.denominator * other + self.numerator;
        self
    }
}

impl Add<Fraction> for Fraction {
    type Output = Fraction;

    fn add(mut self, mut other: Self) -> Self {
        self.simplify();
        other.simplify();
        let lcm = _lcm(self.denominator, other.denominator);
        let self_mul = lcm / self.denominator;
        let other_mul = lcm / other.denominator;
        let mut res = Self {
            numerator: self.numerator * self_mul + other.numerator * other_mul,
            denominator: lcm,
        };
        res.simplify();
        res
    }
}

impl Add<Fraction> for u128 {
    type Output = Fraction;

    fn add(self, mut other: Fraction) -> Fraction {
        other.simplify();
        other.numerator = other.denominator * self + other.numerator;
        other
    }
}

impl Mul<u128> for Fraction {
    type Output = Fraction;

    fn mul(mut self, other: u128) -> Self {
        self.simplify();
        self.numerator = self.numerator * other;
        self.simplify();
        self
    }
}

impl Mul<Fraction> for Fraction {
    type Output = Fraction;

    fn mul(mut self, mut other: Self) -> Self {
        self.simplify();
        other.simplify();
        let mut res = Self {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        };
        res.simplify();
        res
    }
}

impl Mul<Fraction> for u128 {
    type Output = Fraction;

    fn mul(self, mut other: Fraction) -> Fraction {
        other.simplify();
        other.numerator = other.numerator * self;
        other.simplify();
        other
    }
}

impl Div<u128> for Fraction {
    type Output = Fraction;

    fn div(mut self, rhs: u128) -> Self {
        self.simplify();
        self.denominator = self.denominator * rhs;
        self.simplify();
        self
    }
}

impl Div<Fraction> for Fraction {
    type Output = Fraction;

    fn div(mut self, mut rhs: Self) -> Self {
        self.simplify();
        rhs.simplify();
        let mut res = Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        };
        res.simplify();
        res
    }
}

impl Div<Fraction> for u128 {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Fraction {
        let mut res = Fraction {
            numerator: rhs.denominator,
            denominator: rhs.numerator,
        };
        res * self
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == other.numerator * self.denominator
    }
}

impl Eq for Fraction {}

use std::cmp::Ordering;
impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.numerator * other.denominator).cmp(&(other.numerator * self.denominator)))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

use std::fmt;

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl From<u128> for Fraction {
    fn from(n: u128) -> Fraction {
        Fraction {
            numerator: n,
            denominator: 1,
        }
    }
}

#[inline]
fn _gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        _gcd(b, a % b)
    }
}

#[inline]
fn _lcm(a: u128, b: u128) -> u128 {
    let gcd = _gcd(a, b);
    (a / gcd) * b
}