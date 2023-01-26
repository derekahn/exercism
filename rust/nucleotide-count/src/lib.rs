use std::collections::HashMap;

fn is_valid(n: char) -> Result<(), char> {
    let valid = vec!['A', 'C', 'G', 'T'];
    if !valid.contains(&n) {
        return Err(n);
    }
    Ok(())
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    is_valid(nucleotide)?;

    let sequence = dna.chars().into_iter();
    let mut count: usize = 0;
    for n in sequence {
        is_valid(n)?;
        if n == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = "ACGT"
        .chars()
        .map(|c| (c, 0))
        .collect::<HashMap<char, usize>>();

    for n in dna.chars().into_iter() {
        is_valid(n)?;
        match map.get(&n) {
            Some(count) => {
                map.insert(n, count + 1);
            }
            None => {}
        }
    }
    Ok(map)
}
