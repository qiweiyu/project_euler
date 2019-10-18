mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;
mod poker;

extern crate num_bigint;

use num_bigint::BigUint;
use num_bigint::ToBigUint;
use fraction::FractionBig;

fn main() {
    let list = generate(100);
    let len = list.len();
    let mut res = FractionBig::new(list[len - 1].to_biguint().unwrap(), 1.to_biguint().unwrap());
    for i in 2..=len {
        res = list[len - i] + 1 / res;
    }
    res.simplify();
    println!("{:?}", res);
    let (s, _) = res.to_tuple_string();
    let mut sum = 0;
    for ch in s.as_bytes() {
        let ch = *ch as u8 - '0' as u8;
        sum += ch as i32;
    }
    println!("sum: {}", sum);
}

fn generate(limit: usize) -> Vec<u64> {
    let mut res = vec![];
    for i in 0..limit {
        res.push(cal(i) as u64);
    }
    res
}

fn cal(idx: usize) -> usize {
    if idx == 0 {
        2
    } else {
        if idx % 3 == 2 {
            2 * (idx / 3 + 1)
        } else {
            1
        }
    }
}