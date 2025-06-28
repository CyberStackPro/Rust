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
enum IpAddrKind {
    V4,
    V6,
}
// Advanced enum for ai addr
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,                       // No data associated
    Move { x: i32, y: i32 },    // Anonymous struct associated
    Write(String),              // Single String associated
    ChangeColor(i32, i32, i32), // Tuple of i32s associated
}

enum Option<T> {
    None,
    Some(T),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting application."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to R:{}, G:{}, B:{}", r, g, b)
            }
        }
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // we can't just print home and loopback here b/c it is using enum we need to extract from that using match

    let quit_msg = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("hello from Rust"));
    let change_color_msg = Message::ChangeColor(255, 128, 0);

    quit_msg.call();
    move_msg.call();
    write_msg.call();
    change_color_msg.call();

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

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routing  an Ipv4 address...."),
        IpAddrKind::V6 => println!("Routing  an Ipv6 address...."),
    }
}
