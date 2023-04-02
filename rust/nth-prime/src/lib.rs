pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
