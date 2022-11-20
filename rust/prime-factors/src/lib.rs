pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();

    match (2..=n).find(|&x| n % x == 0) {
        Some(p) => {
            primes.push(p);
            primes.append(&mut factors(n / p));
        }
        None => {}
    }

    primes
}
