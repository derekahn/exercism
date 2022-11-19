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
        self.scores().to_vec().pop()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores()
            .iter()
            .map(|&score| score)
            .reduce(|top, score| if top >= score { top } else { score })
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.scores().iter().fold(Vec::new(), |mut accum, &n| {
            if accum.len() < 3 {
                accum.push(n);
            } else if n > accum[2] {
                accum.pop();
                accum.push(n);
            }
            accum.sort_by(|a, b| b.cmp(a));
            accum
        })
    }
}
