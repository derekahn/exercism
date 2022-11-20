pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut idx = n.clone();

    while idx > 1 {
        if let Some(p) = (2..=idx).find(|&x| idx % x == 0) {
            primes.push(p);
            idx /= p;
        }
    }

    primes
}
