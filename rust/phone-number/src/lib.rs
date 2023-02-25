pub fn number(user_number: &str) -> Option<String> {
    let digits: String = user_number
        .chars()
        .filter(|c| c.is_digit(10))
        .enumerate()
        .skip_while(|(i, c)| *i == 0 && *c == '1')
        .map(|(_, c)| c)
        .collect();

    if digits.len() != 10
        || (digits.chars().nth(0).unwrap() as u8) < b'2'
        || (digits.chars().nth(3).unwrap() as u8) < b'2'
    {
        None
    } else {
        Some(digits)
    }
}
