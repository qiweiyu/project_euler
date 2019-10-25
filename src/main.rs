mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;
mod poker;
mod euler;

use euler::phi;
use fraction::Fraction;
use utils::check_permutation;

fn main() {
    let mut res = 0;
    let mut min = Fraction::new(10, 1);
    for i in 2..=500_000 {
        if i % 10_000 == 0 {
            println!("i: {}", i);
        }
        let i = i * 2 - 1;
        let p = phi(i);
        if check_permutation(p as u128, i as u128) {
            let r = Fraction::new(i as u128, p as u128);
            if r < min {
                println!("min: {:?}", r);
                min = r;
                res = i;
            }
        }
    }
    println!("{}", res);
}