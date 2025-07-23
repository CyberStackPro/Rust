// RefCell<T>` - Interior Mutability (Single-Threaded)
// RefCell<T>` is a smart pointer that enables **interior mutability** for data in a **single-threaded context**. It achieves this by moving the borrowing rules from compile time to **runtime**.

use std::{cell::RefCell, fmt::format};

#[derive(Debug)]
struct BankAccount {
    balance: RefCell<i32>,
}
impl BankAccount {
    fn new(initial_balance: i32) -> BankAccount {
        BankAccount {
            balance: RefCell::new(initial_balance),
        }
    }

    fn deposit(&self, amount: i32) {
        // This will panic if another mutable borrow is active.
        *self.balance.borrow_mut() += amount;
        println!(
            "Deposited {}. New Balance: {}",
            amount,
            self.balance.borrow()
        );
    }

    fn withdraw(&self, amount: i32) -> Result<(), String> {
        let mut balance_mut = self.balance.borrow_mut();

        if *balance_mut >= amount {
            *balance_mut -= amount;
            println!("Withdraw {}. New Balance: {}", amount, *balance_mut);
            Ok(())
        } else {
            Err(format!(
                "Insufficient funds. Current balance: {}, Requested amount: {}",
                *balance_mut, amount
            ))
        }
    }
    fn get_balance(&self) -> i32 {
        *self.balance.borrow()
    }
}

fn main() {
    println!("\n--- RefCell<T> for Interior Mutability ---");

    let account = BankAccount::new(100);

    println!("Initial balance: {}", account.get_balance());
}
