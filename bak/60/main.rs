mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;
mod poker;

use crate::primes::is_prime;
use crate::primes::generate_primes;
use crate::utils::select;
use std::collections::HashMap;

fn main() {
    const LIMIT: usize = 10_000;
    const LEN: usize = 5;
    let p_list = generate_primes(LIMIT);
    let mut pair_map = HashMap::new();
    for i in 0..p_list.len() - 1 {
        let p1 = p_list[i];
        println!("generate: {}", p1);
        for j in i + 1..p_list.len() {
            let p2 = p_list[j];
            let join1 = join(p1, p2);
            let join2 = join(p2, p1);
            let is_join1 = if join1 > LIMIT {
                is_prime(join1 as i64)
            } else {
                p_list.contains(&join1)
            };
            let is_join2 = if join2 > LIMIT {
                is_prime(join2 as i64)
            } else {
                p_list.contains(&join2)
            };
            if is_join1 && is_join2 {
                pair_map.entry(p1)
                    .and_modify(|x: &mut (Vec<usize>, Vec<usize>)| {
                        if p1 > p2 {
                            x.0.push(p2)
                        } else {
                            x.1.push(p2)
                        }
                    })
                    .or_insert_with(|| {
                        let mut res: (Vec<usize>, Vec<usize>) = (vec![], vec![]);
                        if p1 > p2 {
                            res.0.push(p2)
                        } else {
                            res.1.push(p2)
                        }
                        res
                    });
                pair_map.entry(p2)
                    .and_modify(|x: &mut (Vec<usize>, Vec<usize>)| {
                        if p2 > p1 {
                            x.0.push(p1)
                        } else {
                            x.1.push(p1)
                        }
                    })
                    .or_insert_with(|| {
                        let mut res: (Vec<usize>, Vec<usize>) = (vec![], vec![]);
                        if p2 > p1 {
                            res.0.push(p1)
                        } else {
                            res.1.push(p1)
                        }
                        res
                    });
            }
        }
    }
    let mut min_sum = std::usize::MAX;
    let mut min_group = vec![];
    for p in &p_list {
        if let Some(pairs) = pair_map.get(p) {
            println!("cal {}", p);
            if pairs.1.len() < LEN - 1 {
                continue;
            }
            let all_group = select(&pairs.1, LEN - 1);
            for group in all_group {
                let mut pass = true;
                for i in 0..group.len() - 1 {
                    let p1 = group[i];
                    let p1_pairs = &pair_map[&p1];
                    for j in i + 1..group.len() {
                        let p2 = group[j];
                        if p2 > p1 {
                            if !p1_pairs.1.contains(&p2) {
                                pass = false;
                                break;
                            }
                        } else {
                            if !p1_pairs.0.contains(&p2) {
                                pass = false;
                                break;
                            }
                        }
                    }
                    if !pass {
                        break;
                    }
                }
                if pass {
                    let sum = group.clone().into_iter().fold(0, |sum, x| { sum + x }) + *p;
                    println!("{} {:?} {}", p, group, sum);
                    if min_sum > sum {
                        min_sum = sum;
                        min_group = group;
                        min_group.push(*p);
                    }
                }
            }
        }
    }
    println!("found {} {:?}", min_sum, min_group)
}

fn join(n1: usize, n2: usize) -> usize {
    let mut res = n1;
    let mut p = 1;
    loop {
        res = res * 10;
        p = p * 10;
        if n2 < p {
            break;
        }
    }
    res = res + n2;
    res
}