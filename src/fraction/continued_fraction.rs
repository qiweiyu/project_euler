pub fn square_root(n: u64) -> (u64, Vec<u64>) {
    let root = (n as f64).sqrt().floor() as u64;
    if n == root * root {
        return (root, vec![]);
    }
    let mut a = root;
    let mut b = 1;
    let mut k = n - a * a;
    let mut list = vec![];
    loop {
        let a1 = (root + a) / k;
        list.push(a1);
        if k == 1 {
            break;
        }
        b = k;
        a = a1 * k - a;
        k = (n - a * a) / b;
    }
    (root, list)
}