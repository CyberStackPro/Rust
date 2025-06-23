#![allow(unused)]

use rand::Rng;
// use std::fs::File;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    // println!("what is your name");
    // let mut name = String::new();
    // let greeting = "Nice to meet you!";
    // println!("Enter your name:");
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");
    // println!("Hello,{}  {}!", name, greeting);

    // print_data_types();
    // random_number();
    loop_example()
}

// rust constant
fn constant() {
    const MAX_POINTS: u32 = 100_000;
    const PI: f32 = 3.14159;
    let age: &str = "25";
    let mut age: u32 = age.trim().parse().expect("Not a number!");
    age = age + 1;
    println!("Max points: {}, PI: {}, Age: {}", MAX_POINTS, PI, age);
}

fn print_data_types() {
    let a: i32 = 10;
    let b: f64 = 20.5;
    let c: char = 'A';
    let d: bool = true;
    let e: &str = "Hello, Rust!";

    println!(
        "Integer: {}, Float: {}, Char: {}, Bool: {}, String: {}",
        a, b, c, d, e
    );

    // Unsigned and signed integers
    println!("Unsigned 8-bit: {}, Signed 8-bit: {}", u8::MAX, i8::MIN);
    println!("Unsigned 16-bit: {}, Signed 16-bit: {}", u16::MAX, i16::MIN);
    println!("Unsigned 32-bit: {}, Signed 32-bit: {}", u32::MAX, i32::MIN);
    println!("Unsigned 64-bit: {}, Signed 64-bit: {}", u64::MAX, i64::MIN);
    println!(
        "Unsigned 128-bit: {}, Signed 128-bit: {}",
        u128::MAX,
        i128::MIN
    );
    println!("Isize: {}, Usize: {}", isize::MAX, usize::MAX);
    println!(
        "Pointer size: {}, Pointer size (unsigned): {}",
        std::mem::size_of::<*const ()>(),
        std::mem::size_of::<*mut ()>()
    );
    // Floating point types
    println!("f32: {}, f64: {}", f32::MAX, f64::MAX);
    // Boolean type
    println!("Boolean: {}", d);
    // Character type
    println!("Character: {}", c);
    // String type
    println!("String: {}", e);
    // Tuple type
    let tuple: (i32, f64, char) = (a, b, c);
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    // Array type
    let array: [i32; 3] = [1, 2, 3];
    println!("Array: [{}, {}, {}]", array[0], array[1], array[2]);
    // Slice type
    let slice: &[i32] = &array[0..2];
    println!("Slice: [{}, {}]", slice[0], slice[1]);
    // String slice type
    let string_slice: &str = "Hello, Rust!";
    println!("String Slice: {}", string_slice);
    // Option type
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    println!(
        "Option Some: {:?}, Option None: {:?}",
        some_value, none_value
    );
    // Result type
    let result_ok: Result<i32, &str> = Ok(100);
    let result_err: Result<i32, &str> = Err("Error occurred");
    println!("Result Ok: {:?}, Result Err: {:?}", result_ok, result_err);
    // Function type
    let add = |x: i32, y: i32| -> i32 { x + y };
    let sum = add(5, 10);
    println!("Function Result: {}", sum);
    // Closure type
    let closure = |x: i32| -> i32 { x * 2 };
    let closure_result = closure(5);
    println!("Closure Result: {}", closure_result);
    // Struct type
    struct Person {
        name: String,
        age: u32,
    }
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Struct Person: Name: {}, Age: {}", person.name, person.age);
    // Enum type
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("Direction: Up"),
        Direction::Down => println!("Direction: Down"),
        Direction::Left => println!("Direction: Left"),
        Direction::Right => println!("Direction: Right"),
    }
    // Trait type
    trait Printable {
        fn print(&self);
    }
    impl Printable for Person {
        fn print(&self) {
            println!("Printable Trait: Name: {}, Age: {}", self.name, self.age);
        }
    }
    let printable_person = Person {
        name: String::from("Bob"),
        age: 25,
    };
    printable_person.print();
    // Box type
    let boxed_value: Box<i32> = Box::new(42);
    println!("Boxed Value: {}", boxed_value);
}

fn math() {
    // Basic arithmetic operations
    let a = 10;
    let b = 5;
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    let remainder = a % b;

    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}",
        sum, difference, product, quotient, remainder
    );

    // Floating point operations
    let x = 5.0;
    let y = 2.0;
    let float_sum = x + y;
    let float_difference = x - y;
    let float_product = x * y;
    let float_quotient = x / y;

    println!(
        "Float Sum: {}, Float Difference: {}, Float Product: {}, Float Quotient: {}",
        float_sum, float_difference, float_product, float_quotient
    );
}

fn random_number() {
    // Generate a random number between 1 and 100
    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(1..=100);
    println!("Random number: {} ", random_number);
}

fn match_example() {
    let number = 5;
    let age2 = 25;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Not one, two, three, four, or five"),
    }
    match age2 {
        0..=17 => println!("You are a minor"),
        18..=64 => println!("You are an adult"),
        _ => println!("You are a senior citizen"),
    }

    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("You are not old enough to vote."),
        Ordering::Equal => println!("You are exactly old enough to vote!"),
        Ordering::Greater => println!("You are old enough to vote!"),
        // std::cmp::Ordering::Less => println!("You are not old enough to vote."),
        // std::cmp::Ordering::Equal => println!("You are exactly old enough to vote!"),
        // std::cmp::Ordering::Greater => println!("You are old enough to vote!"),
    }
}

fn array_example() {
    // Array declaration
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);

    // Accessing elements
    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);

    // Iterating over an array
    for number in &numbers {
        println!("Number: {}", number);
    }

    // Array length
    println!("Array length: {}", numbers.len());
}
fn loop_example() {
    let arr2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;
    // Looping with a while loop
    // loop {
    //     if arr2[loop_index] % 2 == 0 {
    //         loop_index += 1;
    //         continue;
    //     }
    //     if arr2[loop_index] == 9 {
    //         break;
    //     }
    //     println!("Loop index: {}", arr2[loop_index]);
    //     loop_index += 1;
    // }

    // Looping with a for loop
    for i in arr2.iter() {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        if *i == 9 {
            break; // Exit loop when reaching 9
        }
        println!("For loop index: {}", i);
    }

    // Looping with a while loop
    // let mut count = 0;
    // while count < 5 {
    //     println!("Count: {}", count);
    //     count += 1;
    // }

    // Looping with a loop
    // let mut index = 0;
    // loop {
    //     if index >= 5 {
    //         break;
    //     }
    //     println!("Loop index: {}", index);
    //     index += 1;
    // }
}
