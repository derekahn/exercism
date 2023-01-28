#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.trim()
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                'A' | 'C' | 'G' | 'T' => Ok(c),
                _ => Err(i),
            })
            .collect::<Result<String, usize>>()
            .map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        let transcribed: String = self
            .0
            .chars()
            .map(|n| match n {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();

        Rna(transcribed)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.trim()
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                'A' | 'C' | 'G' | 'U' => Ok(c),
                _ => Err(i),
            })
            .collect::<Result<String, usize>>()
            .map(Rna)
    }
}
