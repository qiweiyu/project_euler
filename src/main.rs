mod utils;
mod fraction;
mod primes;
mod fib;

use primes::is_prime;
use primes::generate_primes;
use utils::num_to_list;
use utils::list_to_num;
use utils::get_all_order;

fn main() {
    println!("{:?}", generate_primes(7));
    println!("{:?}", generate_primes(10));
    println!("{:?}", generate_primes(97));
    println!("{:?}", generate_primes(100));
}