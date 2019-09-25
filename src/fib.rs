extern crate num_bigint;

use std::collections::HashMap;
use num_bigint::BigUint;
use num_bigint::ToBigUint;

pub struct Fib {
    cache: HashMap<usize, BigUint>
}

impl Fib {
    #[inline]
    pub fn new() -> Self {
        let mut cache = HashMap::new();
        cache.insert(1, 1.to_biguint().unwrap());
        cache.insert(2, 1.to_biguint().unwrap());
        Fib {
            cache
        }
    }

    pub fn get_by_idx(&mut self, idx: usize) -> BigUint {
        match self.cache.get(&idx) {
            Some(n) => n.clone(),
            None => {
                let res = self.get_by_idx(idx - 1) + self.get_by_idx(idx - 2);
                self.cache.insert(idx, res.clone());
                res
            }
        }
    }
}