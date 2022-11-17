use std::slice::Iter;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ResistorColor {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
}

impl ResistorColor {
    pub fn iterator() -> Iter<'static, ResistorColor> {
        use ResistorColor::*;

        static RC: [ResistorColor; 10] = [
            Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White,
        ];

        RC.iter()
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    for (i, e) in ResistorColor::iterator().enumerate() {
        if color == *e {
            return i as u32;
        }
    }
    return 0;
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        0 => String::from("Black"),
        1 => String::from("Brown"),
        2 => String::from("Red"),
        3 => String::from("Orange"),
        4 => String::from("Yellow"),
        5 => String::from("Green"),
        6 => String::from("Blue"),
        7 => String::from("Violet"),
        8 => String::from("Grey"),
        9 => String::from("White"),
        _ => String::from("value out of range"),
    }
}
pub fn colors() -> Vec<ResistorColor> {
    let mut colors: Vec<ResistorColor> = vec![];
    for &color in ResistorColor::iterator() {
        colors.push(color);
    }
    colors
}
