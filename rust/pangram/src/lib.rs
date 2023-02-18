/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut ascii_codes: Vec<u8> = sentence
        .to_lowercase()
        .chars()
        .map(|c| c as u8)
        .filter(|&c| c > 96 && c < 123)
        .collect();

    ascii_codes.sort();
    ascii_codes.dedup();

    ascii_codes.len() == 26
}
