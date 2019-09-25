mod utils;
mod fraction;
mod primes;
mod fib;

//struct Solution;

//mod list_node;
//use crate::list_node::ListNode;

use std::collections::HashMap;
use std::collections::HashSet;

use fraction::Fraction;
use utils::num_to_list;
use utils::factorial;

fn main() {
    let max = factorial(9) * 7;
    let mut res = vec![];

    for i in 10 .. max {
        let list = num_to_list(i as i32);
        let mut sum = 0;
        for d in list {
            sum += factorial(d as u64);
        }
        if sum == i {
            res.push(sum);
        }
    }

    println!("{:?}", res);
}

/*

fn cal_f(n: usize) -> usize {
    let s = (n as f64).sqrt().ceil() as usize;
    let mut f = vec![1];
    let mut res = 1;
    for i in 2..=s {
        if n % i == 0 {
            let j = n / i;
            if i > j {
                break;
            }
            f.push(i);
            res += i;
            if i < j {
                f.push(j);
                res += j;
            }
        }
    }
    res
}

fn read_file() -> String {
    use std::fs;
    const FILE_NAME: &str = "/Users/qiweiyu/Documents/work/rust/learn/testr/data.txt";
    fs::read_to_string(FILE_NAME).unwrap()
}*/