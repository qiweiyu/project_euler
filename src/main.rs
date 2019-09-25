mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;

use primes::is_prime;
use primes::generate_primes;
use utils::num_to_list;
use utils::list_to_num;
use utils::get_all_order;

use std::collections::HashSet;

fn main() {
    let primes = generate_primes(1_000_000);
    let mut set = HashSet::new();
    for p in &primes {
        set.insert(*p);
    }
    let mut res = vec![];
    for p in &primes {
        println!("{}", p);
        let list = num_to_list(*p as i32);
        let all = get_all_order(&list);
        let mut pass = true;
        for list in all {
            let n = list_to_num(list);
            if !set.contains(&(n as usize)) {
                pass = false;
                break;
            }
        }
        if pass {
            res.push(p);
        }
    }
    println!("{}", res.len());
    println!("{:?}", res)
}