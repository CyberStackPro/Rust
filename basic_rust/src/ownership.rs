// Ownership, Borrowing, and Lifetimes in Rust
// Ownership is a core concept in Rust that ensures memory safety without needing a garbage collector.
// It is based on three main principles: ownership, borrowing, and lifetimes.
//
// 1. Ownership: Each value in Rust has a single owner, which is the variable that holds it.
// When the owner goes out of scope, the value is dropped, and its memory is freed.
// This prevents memory leaks and dangling pointers.
//
// 2. Borrowing: Rust allows you to borrow values using references.
// References can be either mutable or immutable. Immutable references allow you to read a value without taking ownership,
// while mutable references allow you to modify a value. You can have multiple immutable references or one mutable reference at a time,
// but not both simultaneously. This ensures that
// data races are prevented at compile time.

// Example 1: Ownership transfer
fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); // This would cause a compile error
    println!("s2: {}", s2);
}

// Example 2: Borrowing with immutable references
fn borrowing_example() {
    let s1 = String::from("world");
    let len = calculate_length(&s1); // s1 is borrowed immutably
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Example 3: Borrowing with mutable references
fn mutable_borrowing_example() {
    let mut s = String::from("hello");
    change(&mut s); // s is borrowed mutably
    println!("Changed string: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// Example 4: Lifetimes (basic)
fn lifetime_example() {
    let r;
    {
        let x = 5;
        r = &x;
        // println!("r: {}", r); // This would cause a compile error: `x` does not live long enough
    }
    // println!("r: {}", r); // Uncommenting this line will cause a compile error
}

// To run the examples, call them from main():
// fn main() {
//     ownership_example();
//     borrowing_example();
//     mutable_borrowing_example();
//     // lifetime_example(); // Uncomment to see the lifetime error
// }
