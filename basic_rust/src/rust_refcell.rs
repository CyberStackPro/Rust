// RefCell<T>` - Interior Mutability (Single-Threaded)
// RefCell<T>` is a smart pointer that enables **interior mutability** for data in a **single-threaded context**. It achieves this by moving the borrowing rules from compile time to **runtime**.

use std::cell::RefCell;

#[derive(Debug)]
struct BankAccount {
    balance: RefCell<i32>,
    tip: RefCell<i8>,
}
impl BankAccount {
    fn new(initial_balance: i32) -> BankAccount {
        let tip: i8 = 4;
        BankAccount {
            balance: RefCell::new(initial_balance),
            tip: RefCell::new(tip),
        }
    }

    fn deposit(&self, amount: i32) {
        // This will panic if another mutable borrow is active.
        *self.balance.borrow_mut() += amount;
        *self.tip.borrow_mut() += 20;
        println!(
            "Deposited {}. New Balance: {}, And here is tip {}",
            amount,
            self.balance.borrow(),
            self.tip.borrow(),
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

    account.deposit(200);

    let _ = account.withdraw(50);

    // runtime panic (if uncommented)  b/c we have active run time ref cell
    // let _first_borrow = account.balance.borrow_mut();
    // println!("First mutable borrow active.{}", _first_borrow);
    // let _second_borrow = account.balance.borrow_mut(); // This line would panic!
    // println!("Second mutable borrow active. {}", _second_borrow);

    // Demonstrate `Rc<T>` and `RefCell<T>` together
    use std::rc::Rc;

    println!("\n--- Rc<RefCell<T>> for Shared Mutable Data ---");
    let shared_account = Rc::new(RefCell::new(1000)); // Rc owns RefCell, RefCell owns i32

    println!("Initial shared balance: {}", shared_account.borrow()); // Direct borrow on Rc<RefCell>

    println!("Strong count: {}", Rc::strong_count(&shared_account)); // Output: 1

    let handle1 = Rc::clone(&shared_account);
    let handle2 = Rc::clone(&shared_account);

    println!(
        "Strong count after clones: {}",
        Rc::strong_count(&shared_account)
    );
    // Now, multiple Rc pointers can access and *mutably modify* the same underlying data
    // (but only one mutable borrow from RefCell at a time at runtime).
    // let mut current_balance_ref_mut = handle1.borrow_mut();

    // let new_balance_value = current_balance_ref_mut.checked_sub(200).unwrap_or_else(|| {
    //     println!("Handle 1: Underflow prevented! Balance remains unchanged.");
    //     *current_balance_ref_mut // Return the current balance to prevent negative
    // });

    // *current_balance_ref_mut = new_balance_value;
    // handle1
    //     .borrow_mut()
    //     .replace(handle1.borrow().checked_sub(200).unwrap_or_else(|| {
    //         println!("Handle 1: Underflow prevented!");
    //         *handle1.borrow()
    //     }));

    println!("balance after handle1: {}", shared_account.borrow())
}
