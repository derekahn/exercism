pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if msg.len() <= 1 {
        return "Fine. Be that way!";
    }

    let has_letters = get_letters(msg).iter().any(|c| c.is_alphabetic());

    let is_yelling = get_letters(msg)
        .iter()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase());

    let is_question: bool = match get_letters(msg).pop() {
        Some(c) if c == '?' => true,
        _ => false,
    };

    if is_yelling && is_question && has_letters {
        return "Calm down, I know what I'm doing!";
    }
    if is_yelling && has_letters {
        return "Whoa, chill out!";
    }
    if is_question {
        return "Sure.";
    }
    "Whatever."
}

fn get_letters(msg: &str) -> Vec<char> {
    msg.chars()
        .filter(|c| c.is_alphabetic() || *c == '?')
        .collect()
}
