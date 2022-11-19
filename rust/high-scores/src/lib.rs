#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        return self.scores;
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().iter().last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores().iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_3 = self.scores().to_vec();
        top_3.sort_unstable();
        top_3.into_iter().rev().take(3).collect()
    }
}
