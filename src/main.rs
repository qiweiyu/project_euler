mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;

use crate::utils::change_to_base2;
use crate::utils::i2a;
use crate::utils::a2i;

fn main() {
    let mut res = vec![];
    for i in 1..=1000 {
        let str = i2a(i).as_bytes().to_vec();
        let mut r_str = str.clone();
        r_str.reverse();
        let mut n1_str = str.clone();
        n1_str.append(&mut r_str.clone());
        let mut n2_str = str.clone();
        n2_str.pop();
        n2_str.append(&mut r_str.clone());
        let list = vec![
            a2i(String::from_utf8(n1_str).unwrap()),
            a2i(String::from_utf8(n2_str).unwrap())];
        for n in list {
            if n > 1_000_000 {
                continue;
            }
            let base2 = change_to_base2(n).as_bytes().to_vec();
            let mut r_base2 = base2.clone();
            r_base2.reverse();
            if r_base2 == base2 {
                res.push(n);
            }
        }
    }
    println!("{:?}", res);
    let sum = res.into_iter().fold(0, |acc, x| { acc + &x });
    println!("{}", sum)
}