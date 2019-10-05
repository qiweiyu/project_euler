mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;
mod poker;

use crate::fraction::FractionBig;

fn main() {
    let mut f = FractionBig::from(1);
    let mut count = 0;
    for i in 0..1000 {
        f = next(f);
        let (s1, s2) = f.to_tuple_string();
        if s1.len() > s2.len() {
            count += 1;
        }
    }
    println!("{}", count);
}

fn next(f: FractionBig) -> FractionBig {
    1 * (1 + 1 / (1 + f))
}