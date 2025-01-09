use crate::token::Token;

pub struct DEX {
    token_a: Token,
    token_b: Token,
}

impl DEX {
    pub fn new(token_a: Token, token_b: Token) -> Self {
        DEX { token_a, token_b }
    }

    pub fn swap(&mut self, from_token: &str, to_token: &str, amount: u64) {
        if from_token == self.token_a.get_name() && to_token == self.token_b.get_name() {
            self.token_a.burn(amount);
            self.token_b.mint(amount);
        } else if from_token == self.token_b.get_name() && to_token == self.token_a.get_name() {
            self.token_b.burn(amount);
            self.token_a.mint(amount);
        } else {
            println!("Invalid token pair for swap.");
        }
    }

    pub fn add_liquidity(&mut self, token_name: &str, amount: u64) {
        if token_name == self.token_a.get_name() {
            self.token_a.mint(amount);
        } else if token_name == self.token_b.get_name() {
            self.token_b.mint(amount);
        } else {
            println!("Invalid token name for adding liquidity.");
        }
    }

    pub fn remove_liquidity(&mut self, token_name: &str, amount: u64) {
        if token_name == self.token_a.get_name() {
            self.token_a.burn(amount);
        } else if token_name == self.token_b.get_name() {
            self.token_b.burn(amount);
        } else {
            println!("Invalid token name for removing liquidity.");
        }
    }
}
