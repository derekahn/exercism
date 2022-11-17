// This stub file contains items that aren't used yet; feel free to remove this module attribute

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        (self.health <= 0).then(|| Self {
            health: 100,
            mana: (self.level > 9).then_some(100),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            // No mana pool: no damage, damages player instead
            None => {
                self.health = match self.health {
                    h if h < mana_cost => 0,
                    h => h - mana_cost,
                };
                0
            }

            // Insufficient mana: no damage
            Some(mana) if mana < mana_cost => 0,

            // Sufficient mana: damage is equal to double the cost
            Some(mana) => {
                self.mana.replace(mana - mana_cost);
                2 * mana_cost
            }
        }
    }
}
