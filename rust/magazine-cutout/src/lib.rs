// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
use std::collections::HashMap;

fn index_words<'a>(words: &'a [&'a str]) -> HashMap<&'a str, i32> {
    let mut map = HashMap::new();

    for &word in words {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    map
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mag_words = index_words(magazine);
    let note_words = index_words(note);

    let mut is_possible = false;

    for (key, val) in note_words {
        let mag_word_count = mag_words.get(key).unwrap_or(&0);
        if *mag_word_count < val {
            return false;
        }

        is_possible = true
    }

    is_possible
}
