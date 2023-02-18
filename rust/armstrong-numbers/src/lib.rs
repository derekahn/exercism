pub fn is_armstrong_number(num: u32) -> bool {
    let str: String = num.to_string();
    let len: u32 = str.len() as u32;

    let sum: Option<u32> = str.chars().into_iter().try_fold(0, |acc: u32, n: char| {
        (n as u32 - '0' as u32)
            .checked_pow(len)
            .and_then(|pow| acc.checked_add(pow))
    });

    match sum {
        Some(t) => t == num,
        None => false,
    }
}
