pub fn verse(n: u32) -> String {
    let bottle = if n == 2 { "bottle" } else { "bottles" };

    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} {bottle} of beer on the wall.\n", n - 1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<String>>()
        .join("\n")
}
