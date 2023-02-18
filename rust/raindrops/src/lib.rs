pub fn raindrops(n: u32) -> String {
    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => "PlingPlangPlong".to_owned(),
        (0, 0, _) => "PlingPlang".to_owned(),
        (0, _, 0) => "PlingPlong".to_owned(),
        (_, 0, 0) => "PlangPlong".to_owned(),
        (0, _, _) => "Pling".to_owned(),
        (_, 0, _) => "Plang".to_owned(),
        (_, _, 0) => "Plong".to_owned(),
        (_, _, _) => n.to_string(),
    }
}
