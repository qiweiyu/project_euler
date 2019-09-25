use std::collections::HashMap;

pub fn is_prime(n: i64) -> bool {
    n > 1 && find_factors(n as u64).len() == 0
}

pub fn find_factors(n: u64) -> Vec<u64> {
    let s = (n as f64).sqrt().ceil() as u64;
    let mut f = vec![];
    for i in 2..=s {
        if n % i == 0 {
            let j = n / i;
            if i > j {
                break;
            }
            f.push(i);
            if i < j {
                f.push(j);
            }
        }
    }
    f
}

pub fn find_prime_factors(n: u64) -> HashMap<u64, u64> {
    if n < 2 {
        return HashMap::new();
    }
    let mut res = HashMap::new();
    let mut num = n;
    let mut i = 2;
    while num > 1 {
        if num % i == 0 {
            res.entry(i).and_modify(|x| *x += 1).or_insert(1);
            num = num / i;
        } else {
            i += 1;
        }
    }
    res
}