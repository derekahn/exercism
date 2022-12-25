use std::{char, collections::HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let found = possible_anagrams
        .iter()
        .filter(|&&w| {
            w.to_lowercase() != word.to_lowercase() || w.chars().count() != word.chars().count()
        })
        .filter(|&w| {
            let mut chars: Vec<char> = w.to_lowercase().chars().collect();
            chars.sort_unstable();

            let mut check: Vec<char> = word.to_lowercase().chars().collect();
            check.sort_unstable();

            chars == check
        })
        .map(|&w| w);

    HashSet::from_iter(found)
}
