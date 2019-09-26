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
    let mut q = Queue::new();
    q.put(29);
    q.put(39);
    q.put(31);
    //assert_eq!(q.take(), Some(29));
    //assert_eq!(q.take(), Some(39));
    //assert_eq!(q.take(), Some(31));
}