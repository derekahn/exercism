// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();

    for word in magazine.iter() {
        *map.entry(word).or_insert(0) += 1;
    }

    for word in note.iter() {
        match map.get_mut(word) {
            None | Some(0) => {
                return false;
            }
            Some(x) => *x -= 1,
        }
    }

    return true;
}
