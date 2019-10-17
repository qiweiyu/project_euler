mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;
mod poker;

use fraction::continued_fraction::square_root;

fn main() {
    let mut c = 0;
    for i in 1..=10_000 {
        let res = square_root(i);
        if res.1.len() % 2 == 1 {
            c += 1;
        }
    }
    println!("{}", c);
}