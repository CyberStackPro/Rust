mod cost;
mod error;

use crate::cost::{remaining_tokens, total_cost};
use crate::error::ShopErr;
use std::io::stdin;

fn user_input(input: &mut String) {
    stdin().read_line(input).expect("Failed to get input");
}
fn main() {
    println!("🚀 Welcome to the Token Shop!");

    let tokens = 15;
    println!("You have 💰 {tokens} tokens.");

    let mut s = String::new();
    println!("What would you like to buy? (Enter item quantity): ");
    user_input(&mut s);

    let parsed_input = s.trim().parse::<i32>();

    let qty = match parsed_input {
        Ok(n) => n,
        Err(_) => {
            eprintln!("⚠️  Error: {}", ShopErr::ParseError);
            return;
        }
    };

    println!("🧮 Calculating total cost...");

    match remaining_tokens(qty, tokens) {
        Ok(remaining) => {
            println!(
                "✅ Success! You bought {qty} items for {} tokens.",
                total_cost(qty)
            );
            println!("💸 Remaining tokens: {remaining}");
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }
}
