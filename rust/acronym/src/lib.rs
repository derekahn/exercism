fn is_camel_case(word: &str) -> bool {
    let len = word.trim().len();
    let uppers = word.trim().chars().filter(|c| c.is_uppercase()).count();
    let lowers = word.trim().chars().filter(|c| c.is_lowercase()).count();

    len > uppers && len > lowers && lowers > 0 && uppers > 0
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c| c == '-' || c == ' ')
        .fold(String::new(), |mut accum, word| {
            if !word.is_empty() {
                if is_camel_case(word) {
                    word.trim()
                        .chars()
                        .filter(|c| c.is_uppercase())
                        .for_each(|c| accum.push(c));
                } else {
                    let letter = word
                        .trim()
                        .chars()
                        .filter(|c| c.is_alphabetic())
                        .nth(0)
                        .unwrap();

                    accum.push(letter);
                }
            }

            accum
        })
        .to_uppercase()
}
