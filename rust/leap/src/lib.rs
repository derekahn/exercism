pub fn is_leap_year(year: u64) -> bool {
    match (year % 400, year % 100, year % 4) {
        (0, _, 0) => true,
        (_, 0, _) => false,
        (_, _, 0) => true,
        _ => false,
    }
}
