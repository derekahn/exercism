use int_enum::IntEnum;
use std::slice::Iter;

#[repr(u32)]
#[derive(Debug, PartialEq, Clone, Copy, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl ResistorColor {
    pub fn iterator() -> Iter<'static, ResistorColor> {
        use ResistorColor::*;

        static COLORS: [ResistorColor; 10] = [
            Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White,
        ];

        COLORS.iter()
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors: Vec<ResistorColor> = vec![];
    for &color in ResistorColor::iterator() {
        colors.push(color);
    }
    colors
}
