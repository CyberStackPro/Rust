mod auth;
mod cost;
mod error;

use crate::auth::{User, login, register};
use crate::cost::{remaining_tokens, total_cost};
use std::io::{Write, stdin, stdout};

fn get_input(prompt: &str) -> String {
    // stdin().read_line(input).expect("Failed to read input");
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // let mut user_db: Vec<String> = Vec::new();
    // let mut users: Vec<(String, String, String)> = Vec::new();
    let mut users_db: Vec<User> = Vec::new();

    loop {
        let action = get_input("Do you want to [login] or [register]? ");

        match action.to_lowercase().as_str() {
            "login" => {
                let email = get_input("Enter your email: ");
                let password = get_input("Enter your password: ");
                match login(&email, &password, &users_db) {
                    Ok(user) => {
                        println!("✅ Welcome back, {}!", user.username);
                        break;
                    }
                    Err(e) => println!("❌ {}", e),
                }
            }
            "register" => {
                let username = get_input("Enter your username: ");
                let email = get_input("Enter your email: ");
                let password = get_input("Enter your password: ");

                match register(username, email, password, &mut users_db) {
                    Ok(user) => {
                        println!("✅ Account created for {}!", user.username);
                        // break;
                    }
                    Err(e) => println!("❌ {}", e),
                }
            }
            _ => println!("❓ Invalid option. Please type login or register."),
        }

        // authenticate_and_create(&username, &email, &password, &mut users_db).unwrap();

        println!("\n🧾 Current Users:");
        for (i, user) in users_db.iter().enumerate() {
            println!(
                "{}. Username: {}, Email: {}, Password: {}",
                i + 1,
                user.username,
                user.email,
                user.password
            );
        }

        println!("\n---\n");
    }

    // println!("Welcome to the Token Shop!");

    // let mut tokens = 50;
    // println!("You currently have {tokens} tokens.");

    // loop {
    //     let mut s = String::new();
    //     println!("\nEnter the quantity of items you want to buy (or type 'q' to quit): ");
    //     user_input(&mut s);

    //     let input = s.trim();

    //     if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("exit") {
    //         println!("Exiting the shop. Thank you for visiting.");
    //         break;
    //     }

    //     let parsed_input = input.parse::<i32>();
    //     let qty = match parsed_input {
    //         Ok(n) => n,
    //         Err(_) => {
    //             eprintln!("Invalid input. Please enter a valid number.");
    //             continue;
    //         }
    //     };

    //     println!("Calculating total cost...");

    //     match remaining_tokens(qty, tokens) {
    //         Ok(remaining) => {
    //             println!(
    //                 "Purchase successful. You bought {qty} items for {} tokens.",
    //                 total_cost(qty)
    //             );
    //             tokens = remaining;
    //             println!("Remaining tokens: {remaining}");

    //             if tokens == 0 {
    //                 println!("You have used all your tokens. Session ended.");
    //                 break;
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Transaction failed: {}", e);
    //         }
    //     }
    // }
}
