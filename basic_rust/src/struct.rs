// What is a struct in Rust?
// A struct (short for "structure") is a custom data type that lets you group related values together.
// Structs are used to create complex types that have named fields, similar to "objects" in other languages.

// Example: Defining and using a struct

// Define a struct called Person
struct Person {
    name: String,
    age: u32,
    height: f32,
}

// fn main() {
//     let person = Person {
//         name: String::from("Alice"),
//         age: 30,
//         height: 1.65,
//     };
//     println!("Name: {}", person.name);
//     println!("Age: {}", person.age);
//     println!("Height: {} m", person.height);
// }

fn main() {
    let mut account = BankAccount {
        owner: "John".to_string(),
        balance: 150.55,
    };

    // Immutable borrow to check balance
    account.check_balance();

    // Mutable  borrow to withdraw money
    account.withdraw(50.5);

    // Immutable borrow to check balance
    account.check_balance();
}
struct BankAccount {
    owner: String,
    balance: f64,
}
impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {} ",
            amount, self.owner
        );
        self.balance -= amount
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {} ",
            self.owner, self.balance
        );
    }
}
