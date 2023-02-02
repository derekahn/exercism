// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    coordinates: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            coordinates: (x, y),
            direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(&mut self) -> Self {
        let facing = match self.direction() {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Self {
            coordinates: self.coordinates,
            direction: facing,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let facing = match self.direction() {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };

        Self {
            coordinates: self.coordinates,
            direction: facing,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = match self.direction {
            Direction::East => (self.coordinates.0 + 1, self.coordinates.1),
            Direction::North => (self.coordinates.0, self.coordinates.1 + 1),
            Direction::South => (self.coordinates.0, self.coordinates.1 - 1),
            Direction::West => (self.coordinates.0 - 1, self.coordinates.1),
        };

        Self {
            coordinates: (x, y),
            direction: self.direction,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .trim()
            .chars()
            .into_iter()
            .fold(self, |mut acc, instruction| {
                let next = match instruction {
                    'L' => acc.turn_left(),
                    'R' => acc.turn_right(),
                    'A' => acc.advance(),
                    _ => unreachable!(),
                };
                next
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.coordinates.0, self.coordinates.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
