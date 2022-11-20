pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&a| {
            !factors
                .iter()
                .filter(|&b| *b > 0 && a % b == 0)
                .collect::<Vec<&u32>>()
                .is_empty()
        })
        .fold(0, |sum, n| sum + n)
}
