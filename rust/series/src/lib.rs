pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return (0..digits.len()).map(|_| "".to_string()).collect();
    }
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|w| w.iter().collect())
        .collect()
}
