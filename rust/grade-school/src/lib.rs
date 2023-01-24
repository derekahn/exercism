// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    students: Vec<(u32, String)>,
}

impl School {
    pub fn new() -> School {
        Self {
            students: Vec::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.push((grade, student.into()))
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut request: Vec<u32> = self.students.clone().into_iter().map(|(g, _)| g).collect();
        request.dedup();
        request.sort();
        request
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut request: Vec<String> = self
            .students
            .clone()
            .into_iter()
            .filter(|s| s.0 == grade)
            .map(|(_, s)| s)
            .collect();

        request.sort();
        request
    }
}
