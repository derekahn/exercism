pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.iter()
        .zip(list.iter().skip(1))
        .map(|(&w1, &w2)| format!("For want of a {} the {} was lost.\n", w1, w2))
        .collect::<String>()
        + &format!("And all for the want of a {}.", list[0])
}
