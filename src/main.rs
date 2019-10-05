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
    let mut count = 0;
    for line in s.lines() {
        let mut p_card_list = vec![];
        let mut card_list = vec![];
        let mut prev_char = ' ';
        for (pos, ch) in line.chars().enumerate() {
            match pos % 3 {
                0 => {
                    prev_char = ch;
                }
                1 => {
                    card_list.push(Card::new(prev_char, ch));
                }
                _ => {}
            };
            if card_list.len() == 5 {
                p_card_list.push(card_list);
                card_list = vec![];
            }
        }
        let p1 = Poker::new(p_card_list[0].clone());
        let p2 = Poker::new(p_card_list[1].clone());
        if p1 > p2 {
            count += 1;
        }
    }
    println!("{}", count);
}