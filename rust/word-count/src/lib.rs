use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    words
        .trim()
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|w| !w.is_empty())
        .for_each(|w| {
            let word = w
                .trim_start_matches(|c: char| !c.is_alphanumeric())
                .trim_end_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
                .to_string();

            if word.len() > 0 {
                let count = map.entry(word).or_insert(0);
                *count += 1;
            }
        });

    map
}
