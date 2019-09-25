use crate::primes;

pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        assert_ne!(denominator, 0);
        Fraction {
            numerator,
            denominator,
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == other.numerator * self.denominator
    }
}

impl Eq for Fraction {}

use std::fmt;

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

pub fn cal_recurring_cycle(n: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut res_list = vec![];
    let mut r_list = vec![];
    let mut r_map = HashMap::new();

    let mut num = 10;
    loop {
        let rem = num % n;
        let res = num / n;
        if rem == 0 {
            break;
        }
        if r_map.contains_key(&rem) {
            let mut start = false;
            for r1 in r_list {
                start = (r1 == res) || start;
                if start {
                    res_list.push(r1);
                }
            }
            break;
        } else {
            r_map.insert(rem, res);
            r_list.push(res);
            num = rem * 10;
        }
    }
    println!("{:?}", res_list);
    res_list
}