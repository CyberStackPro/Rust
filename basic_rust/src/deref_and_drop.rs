use std::ops::Deref;

/*
 Concept:
    Drop is a trait that lets you define what happens when a value goes out of scope (is destroyed).

 Think of it like:
    A destructor or cleanup method in other languages (C++/Java).


*/
struct Goodbye;

impl Drop for Goodbye {
    fn drop(&mut self) {
        println!("Dropping Goodbye!");
    }
}
fn main() {
    // let g = Goodbye;
    // println!("Doing stuff...");
    let x = String::from("hello");
    println!("this is name: {} ", &x);
} // ← here `g` goes out of scope

// Output:
// Doing stuff...
// Dropping Goodbye!
// struct MyBox<T>(T);

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     let x = MyBox("hello");
//     // Works because of Deref!
//     println!("{}", *x);
// }
