// Vectors in Rust
//         Err(e) => eprintln!("An error occurred: {}", e),
//     }
// Vectors are a dynamic array type in Rust that can grow or shrink in size.
// They are part of the standard library and provide a flexible way to store collections of data.
// Vectors are implemented as a contiguous growable array type, meaning they can store elements of the same type and can change size at runtime.
// Vectors are defined using the `Vec<T>` type, where `T` is the type of elements stored in the vector.
use std::fmt::Debug;
use std::vec::Vec;
// Vectors are useful when you need a collection that can change size, such as when you don't know the number of elements in advance or when you need to add or remove elements dynamically.
// They are commonly used for storing lists, arrays, and other collections of data that need to be manipulated at runtime.
// Vectors are implemented as a contiguous growable array type, meaning they can store elements of the same type and can change size at runtime.
// Vectors are defined using the `Vec<T>` type, where `T` is the type of elements stored in the vector.

// Example: Creating and using a vector
fn main() {
    // 1. Creating an empty vector:
    // Type annotation is often needed initially if no values are pushed immediately.
    let mut v: Vec<i32> = Vec::new();

    // Add elements to the vector
    v.push(10);
    v.push(20);
    v.push(30);

    // or adding string

    // Print the vector
    println!("Numbers: {:?}", v);

    // Access elements in the vector
    if let Some(first) = v.get(0) {
        println!("First number: {}", first);
    }

    // Iterate over the vector
    for number in &v {
        println!("Number: {}", number);
    }

    // Remove an element from the vector
    if let Some(removed) = v.pop() {
        println!("Removed number: {}", removed);
    }

    // Print the vector after removal
    println!("Numbers after removal: {:?}", v);
}
// Vectors can also be created using the `vec!` macro, which allows you to initialize a vector with a set of values.
// Example: Creating a vector using the `vec!` macro
fn create_vector() {
    // Create a vector with initial values
    let fruits: Vec<&str> = vec!["Apple", "Banana", "Cherry"];

    // Print the vector
    println!("Fruits: {:?}", fruits);
}
// Vectors can store any type that implements the `Clone` and `Debug` traits, allowing you to create collections of complex types as well.
fn create_complex_vector() {
    // Define a struct to store complex data
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    // Create a vector of Person structs
    let mut people: Vec<Person> = Vec::new();

    // Add some people to the vector
    people.push(Person {
        name: String::from("Alice"),
        age: 30,
    });
    people.push(Person {
        name: String::from("Bob"),
        age: 25,
    });

    // Print the vector of people
    println!("People: {:?}", people);
}
// Vectors can also be used with generic types, allowing you to create collections of any type that implements the required traits.
fn create_generic_vector<T: Debug + Clone>(items: Vec<T>) {
    // Print the generic vector
    println!("Generic Vector: {:?}", items);
}
// Example usage of the generic vector function
fn main_generic() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    create_generic_vector(numbers);

    let strings: Vec<String> = vec![String::from("Hello"), String::from("World")];
    create_generic_vector(strings);
}

fn vec_array_numbers() {
    // Create a vector of integers
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Print the vector
    println!("Numbers: {:?}", numbers);

    // Print number 3 from the vector
    let third: &i32 = &numbers[2];
    println!("Third number: {}", third);

    // Safely access the third number using `get`
    let third: Option<&i32> = numbers.get(2);
    // The `get` method returns an `Option`, which is `Some` if the index exists, or `None` if it does not.
    // Using a match statement to handle the Option

    match third {
        Some(array) => println!("Third number is: {:?}", third),
        None => println!("No third number found in the vector."),
    }

    // Convert the vector to an array (fixed size)
    let array: [i32; 5] = numbers.try_into().expect("Slice with incorrect length");
    // Note: The `try_into` method requires the `TryInto` trait, which is available in the standard library.
    // This will panic if the vector length does not match the array length.
    // If you want to handle the case where the vector length does not match the array length,
    // you can use a match statement or an if let statement to handle the error gracefully.

    // Print the array
    println!("Array: {:?}", array);
}
