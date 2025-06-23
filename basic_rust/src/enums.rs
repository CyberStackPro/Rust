// What are enums in Rust?
// Enums (short for "enumerations") in Rust are a powerful way to define a type that can be one of several different variants.
// Each variant can have its own data and behavior, making enums a versatile tool for modeling complex data structures.
// Enums are particularly useful when you want to represent a value that can take on one of a limited set of possible values, each with potentially different data associated with it.
// #[allow(dead_code)]
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    // Using the Direction enum
    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("Moving Up!"),
        Direction::Down => println!("Moving Down!"),
        Direction::Left => println!("Moving Left!"),
        Direction::Right => println!("Moving Right!"),
    }

    // Using the Message enum
    let message = Message::Move { x: 10, y: 20 };
    match message {
        Message::Quit => println!("Quitting..."),
        Message::Move { x, y } => println!("Moving to coordinates ({}, {})", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }

    // Using the Option enum
    let some_value = Option::Some(42);
    let no_value: Option<i32> = Option::None;

    match some_value {
        Option::Some(v) => println!("Got a value: {}", v),
        Option::None => println!("No value found"),
    }

    match no_value {
        Option::Some(v) => println!("Got a value: {}", v),
        Option::None => println!("No value found"),
    }
}
