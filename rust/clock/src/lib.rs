use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: Self::normalize((60 * hours) + minutes),
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Self {
            minutes: Self::normalize(self.minutes + minutes),
        }
    }

    pub fn hours(&self) -> i32 {
        self.minutes / 60
    }

    pub fn minutes(&self) -> i32 {
        self.minutes % 60
    }

    fn normalize(minutes: i32) -> i32 {
        const MINUTES_PER_DAY: i32 = 1440;

        (MINUTES_PER_DAY + (minutes % MINUTES_PER_DAY)) % MINUTES_PER_DAY
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
