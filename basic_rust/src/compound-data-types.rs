// Compound Data Types in Rust

// Compound data types allow you to create complex data structures by combining multiple values of different types.
// In Rust, the main compound data types are tuples and arrays. Tuples can hold values of different types, while arrays hold values of the same type.
// arras, tuples, slices and strings (slice sting) are the most common compound data types in Rust.

// Arrays
// An array is a fixed-size collection of elements of the same type. The size of the array is part of its type.
// Arrays are defined with square brackets, and the type of the elements is specified before the semicolon.
// Example: let arr: [i32; 3] = [1, 2, 3];

// Tuples
// A tuple is a fixed-size collection of values of different types. Tuples can contain any number of elements, and each element can be of a different type.
// Tuples are defined with parentheses, and you can access their elements using dot notation.
// Example: let tup: (i32, f64, char) = (42, 3.14, 'Z');

fn main() {
    // Tuple: A fixed-size collection of values of different types
    let person: (&str, i32, f64) = ("Alice", 30, 5.5);
    println!(
        "Name: {}, Age: {}, Height: {}",
        person.0, person.1, person.2
    );
    let my_mix_tuple = ("Bob", 25, 6.0, true, [1, 2, 3]);
    println!(
        "My Mix Name: {}, Age: {}, Height: {}, Is Student: {}, Numbers: {:?}",
        my_mix_tuple.0, my_mix_tuple.1, my_mix_tuple.2, my_mix_tuple.3, my_mix_tuple.4
    );

    // Array: A fixed-size collection of values of the same type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    let fruits: [&str; 3] = ["banana", "mango", "orange"];
    println!("Fruits: {:?}", fruits);

    // Accessing elements in a tuple
    let (name, age, height) = person;
    println!("Name: {}, Age: {}, Height: {}", name, age, height);

    // Slicing allows you to create a view into a portion of an array or a string.
    let slice: &[i32] = &numbers[1..4]; // Slicing the array to get elements from index 1 to 3
    println!("Slice of numbers: {:?}", slice);
    let string_slice: &str = &"Hello, Rust!"[7..11]; // Slicing a string
    println!("String slice: {}", string_slice);
    let books_slice: &[String] = &[
        "Rust Programming".to_string(),
        "Learning Rust".to_string(),
        "Advanced Rust".to_string(),
    ][1..3];
    println!("Books Slice: {:?}", books_slice);

    // String Vs String Slices (&str)
    // In Rust, `String` is a [growable,mutable,owned string type], heap-allocated data structure, while `&str` is a string slice that represents a view into a string.
    let mut my_string: String = String::from("Hello, Rust!");
    // let my_string_slice: &str = &my_string[0..5]; // Slicing the String to get a string slice
    my_string.push_str(" World!"); // Modifying the String

    println!("String: {} ", my_string,);

    // B- &str (String Slice)
    // A string slice is a reference to a portion of a string. It is denoted by `&str` and is used to borrow a part of a string without taking ownership.
    let my_string_slice: &str = &my_string[0..5]; // Slicing the String to get a string slice

    // Accessing elements in an array
    for number in numbers.iter() {
        println!("Number: {}", number);
    }
}
