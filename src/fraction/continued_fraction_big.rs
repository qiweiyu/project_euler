use super::fraction_big::FractionBig;

extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, Zero};

#[derive(Clone)]
pub struct ContinuedFractionBig {
    root: u128,
    list: Vec<u128>,
}

impl ContinuedFractionBig {
    pub fn from_square_root(n: u128) -> Self {
        let root = (n as f64).sqrt().floor() as u128;
        if n == root * root {
            return Self {
                root,
                list: vec![],
            };
        }
        let mut a = root;
        let mut b = 1;
        let mut k = n - a * a;
        let mut list = vec![];
        loop {
            let a1 = (root + a) / k;
            list.push(a1);
            if k == 1 {
                break;
            }
            b = k;
            a = a1 * k - a;
            k = (n - a * a) / b;
        }
        Self {
            root,
            list,
        }
    }

    pub fn get_convergent_fraction(&self, idx: usize) -> FractionBig {
        if idx == 0 || self.list.len() == 0 {
            FractionBig::new(BigUint::from(self.root), One::one())
        } else {
            let one: BigUint = One::one();
            let idx = idx - 1;
            let list_len = self.list.len();
            let pos = idx % list_len;
            let mut res = FractionBig::new(BigUint::from(self.list[pos]), one);
            for i in 1..=idx {
                let pos = (idx - i) % list_len;
                res = 1 / res + self.list[pos];
            }
            self.root.clone() + 1 / res
        }
    }
}

use std::fmt;

impl fmt::Debug for ContinuedFractionBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {:?}]", self.root, self.list)
    }
}