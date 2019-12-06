pub fn gcd(n: u64, m: u64) -> u64 {
    if n % m == 0 {
        m
    } else {
        gcd(m, n % m)
    }
}

pub fn lcm(n: u64, m: u64) -> u64 {
    n / gcd(n, m) * m
}
