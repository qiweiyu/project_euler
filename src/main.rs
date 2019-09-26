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
use queue::Queue;

use std::collections::HashSet;

fn main() {
    let primes = generate_primes(1_000_000);
    let mut res = vec![];
    for p in &primes {
        println!("{}", p);
        let mut queue = Queue::from_vec(num_to_list(*p as i32));
        let q1 = queue.clone();
        let len = queue.len();
        let mut pass = true;
        let mut tt = vec![];
        for i in 0..len {
            queue.roll();
            let n = list_to_num(queue.clone().into_vec());
            tt.push(n);
            if !primes.contains(&(n as usize)) {
                pass = false;
                break;
            }
        }
        if pass {
            res.push(p);
            println!("{:?}", tt);
            println!("{:?}", queue);
            println!("{:?}", q1);
            println!("{:?}", num_to_list(*p as i32));
        }
    }
    println!("{:?}", res);
    println!("{}", res.len());
}