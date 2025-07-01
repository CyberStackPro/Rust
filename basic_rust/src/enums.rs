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

// enum Option<T> {
//     None,
//     Some(T),
// }

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

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // println!("--- Routing IP Kinds ---");

    // route(four);
    // route(six);

    let coin = 10;
    let coin_type = match coin {
        1 => "Penny",
        5 => "Nickel",
        10 => "Dime",
        25 => "Quarter",
        _ => "Unknown Coin",
    };
    // println!("\nYou received a {}.", coin_type);

    let unknown_coin = 7;
    let unknown_type = match unknown_coin {
        1 => "Penny",
        5 => "Nickel",
        10 => "Dime",
        25 => "Quarter",
        _ => "Unknown Coin", // Catches 7
    };
    // println!("You received an {}.", unknown_type);
    // println!("--- Processing Messages ---");
    // process_message(Message::Quit);
    // process_message(Message::Move { x: 50, y: -20 });
    // process_message(Message::Write(String::from("System log entry")));
    // process_message(Message::ChangeColor(255, 0, 100));

    // Example of returning a value from a match

    let favorite_color = Message::ChangeColor(255, 165, 0);
    let color_desc = match favorite_color {
        Message::ChangeColor(r, g, b) if r == 255 && g == 165 && b == 0 => {
            "My favorite color (Orange)!"
        }
        Message::ChangeColor(_, _, _) => "Some other color",
        _ => "Not a color message",
    };

    // println!("\nColor description: {}", color_desc);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Result of plus_one(Some(5)): {:?}", six); // Output: Some(6)
    println!("Result of plus_one(None): {:?}", none); // Output: None

    let some_string_option: Option<String> = Some(String::from("Hello, Option!"));
    match some_string_option {
        Some(s) => {
            println!("We received a string: {}", s);
            // `s` has ownership of the String here, `some_string_option` is now None
        }
        None => {
            println!("No string was provided.");
        }
    }
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routing  an Ipv4 address...."),
        IpAddrKind::V6 => println!("Routing  an Ipv6 address...."),
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit message received. Exiting program.");
        }
        Message::Move { x, y } => {
            println!("The Move message received. Moving to x={}, y={}", x, y);
        }
        Message::Write(text) => {
            println!("The Write message received. Text: '{}'", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!(
                "The ChangeColor message received. Color: R={}, G={}, B={}",
                r, g, b
            );
        }
    }
}
