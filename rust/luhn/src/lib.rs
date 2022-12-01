/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let seq = code.replace(" ", "");

    if seq.len() < 2 || !seq.chars().all(char::is_numeric) {
        return false;
    }

    seq.chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, n)| match i % 2 {
            0 => n,
            _ if n == 9 => n,
            _ => (n * 2) % 9,
        })
        .sum::<u32>()
        % 10
        == 0
}
