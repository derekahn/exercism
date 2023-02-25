pub fn number(user_number: &str) -> Option<String> {
    let digits: Vec<u32> = user_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    match digits.len() {
        10 => {
            if digits[0] < 2 || digits[3] < 2 {
                None
            } else {
                Some(digits.iter().map(|n| n.to_string()).collect())
            }
        }
        11 if digits[0] == 1 => {
            if digits[1] < 2 || digits[4] < 2 {
                None
            } else {
                Some(digits[1..].iter().map(|n| n.to_string()).collect())
            }
        }
        _ => None,
    }
}
