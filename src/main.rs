mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;
mod poker;

use crate::utils::read_file;
use crate::poker::Card;
use crate::poker::Poker;

fn main() {
    let s = read_file();
    for line in s.lines() {}
    let mut list = vec![
        Card::new('2', 'H'),
        Card::new('3', 'C'),
        Card::new('A', 'D'),
        Card::new('J', 'H'),
        Card::new('J', 'S'),
    ];
    let p = Poker::new(list);
    println!("{:?}", p);
}