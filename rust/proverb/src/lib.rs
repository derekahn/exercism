pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => return String::new(),
        1 => return format!("And all for the want of a {}.", list[0]),
        _ => {
            let mut phrases: Vec<String> = Vec::new();
            let end = list.len() - 1;

            for (i, &word) in list.iter().enumerate() {
                let mut phrase: String = format!("And all for the want of a {}.", list[0]);

                if i != end {
                    phrase = format!("For want of a {} the {} was lost.", word, list[i + 1]);
                }

                phrases.push(phrase);
            }

            phrases.join("\n")
        }
    }
}
