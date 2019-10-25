use crate::primes::find_prime_factors;

pub fn phi(n: u64) -> u64 {
    let prime_map = find_prime_factors(n);
    let mut res = n;
    for (p, pow) in prime_map {
        res = res / p;
        res = res * (p - 1);
    }
    res
}