pub fn cal_recurring_cycle(n: u64) -> Vec<u64> {
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