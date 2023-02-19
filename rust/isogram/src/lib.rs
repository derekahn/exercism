use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let map: HashMap<char, u8> = candidate
        .to_lowercase()
        .chars()
        .into_iter()
        .filter_map(|c| match c {
            'a'..='z' => Some(c),
            _ => None,
        })
        .fold(HashMap::<char, u8>::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    for (_, val) in map {
        if val > 1 {
            return false;
        }
    }
    return true;
}
