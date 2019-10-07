mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;
mod poker;

use crate::utils::get_all_order;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut list = vec![];
    for i in 3..=8 {
        list.push(P::new(i));
    }
    let order_list: Vec<Vec<usize>> = get_all_order(&vec![0, 1, 2, 3, 4, 5]);
    for order in order_list {
        let mut check_list = list.clone();
        let mut pass = false;
        let mut i = 0;
        let mut p0 = check_list[order[0]].clone();
        let mut p1 = check_list[order[1]].clone();
        let mut p2 = check_list[order[2]].clone();
        let mut p3 = check_list[order[3]].clone();
        let mut p4 = check_list[order[4]].clone();
        let mut p5 = check_list[order[5]].clone();
        while !pass {
            i += 1;
            println!("start: {}", i);
            p0.join_suf(&mut p1);
            p1.join_suf(&mut p2);
            p2.join_suf(&mut p3);
            p3.join_suf(&mut p4);
            p4.join_suf(&mut p5);
            p5.join_suf(&mut p0);
            pass = p0.check() && p1.check() && p2.check() && p3.check() && p4.check() && p5.check();
        }
        println!("{:?}", p0);
        println!("{:?}", p1);
        println!("{:?}", p2);
        println!("{:?}", p3);
        println!("{:?}", p4);
        println!("{:?}", p5
        );
    }
}

#[derive(Debug, Clone)]
struct P {
    size: i32,
    prefix: HashSet<i32>,
    suffix: HashSet<i32>,
    pre_map: HashMap<i32, HashSet<i32>>,
    suf_map: HashMap<i32, HashSet<i32>>,
}

impl P {
    fn new(size: i32) -> Self {
        let mut p = P {
            size,
            prefix: HashSet::new(),
            suffix: HashSet::new(),
            pre_map: HashMap::new(),
            suf_map: HashMap::new(),
        };
        p._generate();
        p
    }

    fn join_suf(&mut self, suf: &mut Self) {
        let suf_pre_set = &suf.prefix;
        let cur_suf_set = &self.suffix;
        let diff = Self::_cal_diff(suf_pre_set, cur_suf_set);
        for v in diff {
            suf._rm_pre(&v);
        }
        let suf_pre_set = &suf.prefix;
        let diff = Self::_cal_diff(cur_suf_set, suf_pre_set);
        for v in diff {
            self._rm_suf(&v);
        }
    }

    fn check(&self) -> bool {
        self.prefix.len() <= 1 && self.suffix.len() <= 1
    }

    fn _generate(&mut self) {
        let mut i = 0;
        loop {
            i += 1;
            let res = cal(self.size, i);
            if res < 1000 {
                continue;
            }
            if res > 9999 {
                break;
            }
            let (pre, suf) = (res / 100, res % 100);
            if suf < 10 {
                continue;
            }
            self.prefix.insert(pre);
            self.suffix.insert(suf);
            self.pre_map.entry(pre)
                .and_modify(|x: &mut HashSet<i32>| {
                    x.insert(suf);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(suf);
                    set
                });
            self.suf_map.entry(suf)
                .and_modify(|x: &mut HashSet<i32>| {
                    x.insert(pre);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(pre);
                    set
                });
        }
    }

    fn _rm_pre(&mut self, pre: &i32) {
        if !self.prefix.contains(pre) {
            return;
        }
        self.prefix.remove(pre);
        let suf_set = self.pre_map.remove(pre).unwrap();
        for suf in suf_set {
            let pre_set = self.suf_map.get_mut(&suf).unwrap();
            pre_set.remove(pre);
            if pre_set.len() == 0 {
                self.suf_map.remove(&suf);
                self.suffix.remove(&suf);
            }
        }
    }

    fn _rm_suf(&mut self, suf: &i32) {
        if !self.suffix.contains(suf) {
            return;
        }
        self.suffix.remove(suf);
        let pre_set = self.suf_map.remove(suf).unwrap();
        for pre in pre_set {
            let suf_set = self.pre_map.get_mut(&pre).unwrap();
            suf_set.remove(suf);
            if suf_set.len() == 0 {
                self.pre_map.remove(&pre);
                self.prefix.remove(&pre);
            }
        }
    }

    fn _cal_diff(set_all: &HashSet<i32>, set_part: &HashSet<i32>) -> Vec<i32> {
        let mut res = vec![];
        for v in set_all {
            if set_part.contains(v) {
                continue;
            }
            res.push(*v);
        }
        res
    }
}

fn cal(size: i32, n: i32) -> i32 {
    match size {
        3 => n * (n + 1) / 2,
        4 => n * n,
        5 => n * (3 * n - 1) / 2,
        6 => n * (2 * n - 1),
        7 => n * (5 * n - 3) / 2,
        8 => n * (3 * n - 2),
        _ => n
    }
}