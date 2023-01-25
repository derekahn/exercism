#[allow(clippy::new_without_default)]
use std::collections::BTreeMap;

#[derive(Default)]
pub struct School(BTreeMap<u32, Vec<String>>);

impl School {
    pub fn new() -> School {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0
            .entry(grade)
            .and_modify(|students| students.push(String::from(student)))
            .or_insert(vec![String::from(student)]);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.0.get(&grade) {
            Some(students) => {
                let mut students = students.iter().map(|s| s.clone()).collect::<Vec<String>>();
                students.sort();
                students
            }
            None => vec![],
        }
    }
}
