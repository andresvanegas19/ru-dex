use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    name: String,
    supply: u64,
}

impl Token {
    pub fn new(name: &str, supply: u64) -> Self {
        Token {
            name: name.to_string(),
            supply,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_supply(&self) -> u64 {
        self.supply
    }

    pub fn mint(&mut self, amount: u64) {
        self.supply += amount;
    }

    pub fn burn(&mut self, amount: u64) {
        self.supply -= amount;
    }
}
