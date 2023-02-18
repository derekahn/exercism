pub fn raindrops(n: u32) -> String {
    let mut drops: Vec<String> = Vec::new();
    if n % 3 == 0 {
        drops.push("Pling".to_owned())
    }
    if n % 5 == 0 {
        drops.push("Plang".to_owned())
    }
    if n % 7 == 0 {
        drops.push("Plong".to_owned())
    }
    if drops.len() == 0 {
        return n.to_string();
    }
    drops.join("")
}
