/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let seq = code.replace(" ", "");
    if seq.len() < 2 || !seq.chars().all(char::is_numeric) {
        return false;
    }

    let double = |n: u32| if n * 2 > 9 { n * 2 - 9 } else { n * 2 };

    seq.chars()
        .rev()
        .filter(|c| c.is_ascii_digit())
        .enumerate()
        .map(|(i, n)| match n.to_digit(10) {
            Some(num) => {
                if i % 2 == 1 {
                    double(num)
                } else {
                    num
                }
            }
            None => 0,
        })
        .sum::<u32>()
        % 10
        == 0
}
