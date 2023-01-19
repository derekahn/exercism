#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    fn all() -> [Allergen; 8] {
        use Allergen::*;
        [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
    }
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (allergen.clone() as u32) > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all()
            .into_iter()
            .filter(|alergen| self.is_allergic_to(alergen))
            .collect()
    }
}
