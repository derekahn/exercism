use std::cmp::Ordering;
use std::iter::once;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Frame {
    Strike,
    Spare(u16, u16),
    Open(u16, u16),
}

impl Frame {
    fn expected_extra_rolls(self) -> usize {
        match self {
            Frame::Strike => 2,
            Frame::Spare(_, _) => 1,
            Frame::Open(_, _) => 0,
        }
    }

    fn base_points(self) -> u16 {
        match self {
            Frame::Strike => 10,
            Frame::Spare(_, _) => 10,
            Frame::Open(a, b) => a + b,
        }
    }
}

#[derive(Debug, Default)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    current_pins: Option<u16>,
}

impl BowlingGame {
    fn rolls_iter<'a>(&'a self, frame: usize) -> impl Iterator<Item = u16> + 'a {
        self.frames[frame..]
            .iter()
            .flat_map(|&f| match f {
                Frame::Spare(a, b) => vec![a, b],
                Frame::Strike => vec![10],
                Frame::Open(a, b) => vec![a, b],
            })
            .chain(self.current_pins.into_iter())
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() > 9
            && self.rolls_iter(10).count() >= self.frames[9].expected_extra_rolls()
        {
            return Err(Error::GameComplete);
        }

        match self.current_pins {
            None => match pins.cmp(&10) {
                Ordering::Greater => return Err(Error::NotEnoughPinsLeft),
                Ordering::Equal => self.frames.push(Frame::Strike),
                Ordering::Less => self.current_pins = Some(pins),
            },

            Some(n) => {
                self.current_pins = None;

                match (n + pins).cmp(&10) {
                    Ordering::Greater => return Err(Error::NotEnoughPinsLeft),
                    Ordering::Equal => self.frames.push(Frame::Spare(n, pins)),
                    Ordering::Less => self.frames.push(Frame::Open(n, pins)),
                }
            }
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() < 10
            || self.rolls_iter(10).count() != self.frames[9].expected_extra_rolls()
        {
            return None;
        }

        Some(
            self.frames[..10]
                .iter()
                .map(|f| f)
                .enumerate()
                .flat_map(|(i, f)| {
                    self.rolls_iter(i + 1)
                        .take(f.expected_extra_rolls())
                        .chain(once(f.base_points()))
                })
                .sum(),
        )
    }
}
