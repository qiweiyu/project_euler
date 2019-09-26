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
    let mut q: Queue<i32> = Queue::new();
    q.insert(29);
    q.insert(39);
    q.insert(31);
    //assert_eq!(q.take(), Some(29));
    //assert_eq!(q.take(), Some(39));
    //assert_eq!(q.take(), Some(31));
}