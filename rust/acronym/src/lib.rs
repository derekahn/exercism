pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            let inner_upper_letters = word
                .chars()
                .skip_while(|c| c.is_uppercase())
                .filter(|c| c.is_uppercase());

            word.chars().take(1).chain(inner_upper_letters)
        })
        .collect::<String>()
        .to_uppercase()
}
