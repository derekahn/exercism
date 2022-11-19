fn bottles(num: u32) -> String {
    match num {
        0 => String::from("No more bottles"),
        1 => String::from("1 bottle"),
        _ => format!("{num} bottles"),
    }
}

pub fn verse(n: u32) -> String {
    let line1 = format!("{} of beer on the wall, ", bottles(n));
    let line2 = format!("{} of beer.\n", bottles(n).to_lowercase());

    let line3 = if n == 0 {
        String::from("Go to the store and buy some more, ")
    } else {
        let subject = if n == 1 {
            String::from("it")
        } else {
            String::from("one")
        };
        format!("Take {subject} down and pass it around, ")
    };

    let line4 = {
        let remainder = if n == 0 { 99 } else { n - 1 };

        format!(
            "{} of beer on the wall.\n",
            bottles(remainder).to_lowercase()
        )
    };

    format!("{line1}{line2}{line3}{line4}")
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses: Vec<String> = Vec::new();
    let mut i = start;

    while i >= end {
        verses.push(verse(i));

        if i == 0 {
            break;
        }

        i -= 1;
    }

    verses.join("\n")
}
