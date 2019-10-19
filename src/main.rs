mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;
mod poker;

extern crate num_bigint;
extern crate num_traits;

use fraction::ContinuedFractionBig;
use fraction::FractionBig;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use utils::i_sqrt;

fn main() {
    let mut max = (0, Zero::zero(), Zero::zero());
    let one: BigUint = One::one();
    for i in 1..=1000 {
        if let Some(v) = i_sqrt(i) {
            continue;
        }
        let cf = ContinuedFractionBig::from_square_root(i as u128);
        let mut j = 0;
        loop {
            j += 1;
            let f = cf.get_convergent_fraction(j);
            println!("{} {} {:?}", i, j, f);
            let (x, y) = f.to_tuple();
            let l_p: BigUint = x.clone() * x.clone() - one.clone();
            let r_p: BigUint = i * y.clone() * y.clone();
            if l_p == r_p {
                println!("find for {} is {} {}", i, x, y);
                if x > max.1 {
                    max = (i, x, y);
                }
                break;
            }
        }
    }
    println!("res: {:?}", max)
}