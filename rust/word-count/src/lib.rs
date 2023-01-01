use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .map(|w| w.trim_matches('\'').to_lowercase())
        .fold(HashMap::new(), |mut map, w| {
            let entry = map.entry(w).or_insert(0);
            *entry += 1;
            map
        })
}
