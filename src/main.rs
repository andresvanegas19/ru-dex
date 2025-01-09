use serde::{Deserialize, Serialize};
use serde_json;

mod token;
mod dex;

fn main() {
    println!("Initializing DEX platform...");

    // Initialize tokens
    let token_a = token::Token::new("TokenA", 1000);
    let token_b = token::Token::new("TokenB", 2000);

    // Initialize DEX
    let mut dex = dex::DEX::new(token_a, token_b);

    // Example usage
    dex.swap("TokenA", "TokenB", 100);
    dex.add_liquidity("TokenA", 500);
    dex.remove_liquidity("TokenB", 300);

    println!("DEX platform initialized.");
}
