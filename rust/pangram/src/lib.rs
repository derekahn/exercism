/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut ascii_codes: Vec<u8> = sentence
        .chars()
        .filter_map(|c| match c {
            'a'..='z' => Some(c as u8),
            'A'..='Z' => Some(c.to_ascii_lowercase() as u8),
            _ => None,
        })
        .collect();

    ascii_codes.sort();
    ascii_codes.dedup();

    ascii_codes.len() == 26
}
