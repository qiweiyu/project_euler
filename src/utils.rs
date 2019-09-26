use std::collections::HashMap;

pub fn hash_map_to_string<T, R>(num: &HashMap<T, R>) -> String
    where T: std::cmp::Ord + std::hash::Hash + std::fmt::Display, R: std::fmt::Display
{
    let mut num_list = vec![];
    for k in num.keys() {
        num_list.push(k);
    }
    num_list.sort();
    let mut s = String::new();
    for k in num_list {
        s = format!("{}:{}_{}", s, k, num.get(&k).unwrap())
    }
    s
}

pub fn select<T>(list: &Vec<T>, n: usize) -> Vec<Vec<T>>
    where T: Clone
{
    if n == 1 {
        let mut res = vec![];
        for v in list {
            res.push(vec![v.clone()])
        }
        return res;
    }
    if n >= list.len() {
        return vec![list.to_vec()];
    }
    let mut res = vec![];
    for i in 0..=list.len() - n {
        let v = &list[i];
        let sub_list_list = select(&list[i + 1..].to_vec(), n - 1);
        for mut sub_list in sub_list_list {
            sub_list.push(v.clone());
            res.push(sub_list);
        }
    }
    res
}

pub fn get_all_order<T>(list: &Vec<T>) -> Vec<Vec<T>>
    where T: Clone
{
    let mut res = vec![];
    if list.len() <= 1 {
        res.push(list.to_vec());
    } else {
        let mut list = list.to_vec();
        for i in 0..list.len() {
            if i > 0 {
                let tmp = list[i].clone();
                list[i] = list[0].clone();
                list[0] = tmp;
            }
            let v = list[0].clone();
            let sub_list_list = get_all_order(&list[1..].to_vec());
            for mut sub_list in sub_list_list {
                sub_list.push(v.clone());
                res.push(sub_list);
            }
            if i > 0 {
                let tmp = list[i].clone();
                list[i] = list[0].clone();
                list[0] = tmp;
            }
        }
    }
    res
}

pub fn a2i(num_str: String) -> i32 {
    let mut res = 0;
    let zero = '0' as u8;
    for i in num_str.as_bytes() {
        res = res * 10;
        res = res + (*i - zero) as i32;
    }
    res
}

pub fn i2a(n: i32) -> String {
    let mut res = vec![];
    let mut num = n;
    let zero = '0' as u8;
    while num > 0 {
        res.push((num % 10) as u8 + zero);
        num = num / 10;
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub fn read_file() -> String {
    use std::fs;
    const FILE_NAME: &str = "/Users/qiweiyu/Documents/work/rust/learn/project_euler/data.txt";
    fs::read_to_string(FILE_NAME).unwrap()
}


pub fn change_to_base2(n: i32) -> String {
    let mut list = vec![];
    let mut num = n;
    let zero = '0' as u8;
    while num > 0 {
        list.push((num & 1) as u8 + zero);
        num = num / 2;
    }
    list.reverse();
    String::from_utf8(list).unwrap()
}