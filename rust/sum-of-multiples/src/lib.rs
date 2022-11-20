pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| factors.iter().any(|&m| m != 0 && n % m == 0))
        .sum()
}
