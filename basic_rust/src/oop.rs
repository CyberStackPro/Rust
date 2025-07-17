// In many programming languages, "objects" combine data and the functions that operate on that data. In Rust, structs hold the data, and **methods** are functions that are associated with a particular struct (or enum). Methods are defined within an `impl` (implementation) block for a given type.

// First, let's define a struct for a Rectangle
#[derive(Debug)] // This macro allows us to print the struct using {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // This is a method. Its first parameter is `&self`,
    // which is an immutable reference to the instance of the struct.
    // It means this method can read the struct's data but not modify it.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // This method takes ownership of `self`.
    // It consumes the instance, meaning the struct instance cannot be used after this call.
    // This is less common but useful for "consuming" operations (e.g., building something).
    fn consume_and_describe(self) -> String {
        format!("Consumed Rectangle: {}x{}", self.width, self.height)
    }

    // This is an "associated function" (often called a static method in other languages).
    // It does NOT take `self` as its first parameter.
    // It's often used as a constructor.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Another associated function that takes references to other Rectangles
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // Calling the `area` method
    );

    // Using the `square` associated function (like a static method)
    let mut sq = Rectangle::square(25); // Call using `StructName::function_name`
    println!("Square: {:?}", sq);

    // Calling a mutable method
    sq.scale(2); // `sq` must be mutable to call a `&mut self` method
    println!("Scaled square: {:?}", sq);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false

    // Calling a method that takes ownership of `self`
    let large_rect = Rectangle {
        width: 100,
        height: 200,
    };
    let description = large_rect.consume_and_describe();
    println!("{}", description);
    // println!("{:?}", large_rect); // ERROR: value borrowed here after move (large_rect was consumed)
}

// Using enums

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => {
                println!("Processing Quit message: Shutting down application.");
            }
            Message::Move { x, y } => {
                println!(
                    "Processing Move message: Moving to coordinates ({}, {}).",
                    x, y
                );
            }
            Message::Write(text) => {
                println!("Processing Write message: Writing text '{}'.", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Processing ChangeColor message: Setting color to RGB({}, {}, {}).",
                    r, g, b
                );
            }
        }
    }

    // This is an associated function (like a static method for enums).
    // It acts as a constructor for a `Move` message.
    fn create_move_message(x_coord: i32, y_coord: i32) -> Message {
        Message::Move {
            x: x_coord,
            y: y_coord,
        }
    }

    // Another example: a method that checks if a message is a Quit message
    fn is_quit(&self) -> bool {
        match self {
            Message::Quit => true,
            _ => false, // Catch-all for any other variant
        }
    }
}

// fn main() {
//     let quit_msg = Message::Quit;
//     let move_msg = Message::create_move_message(10, 20); // Using the associated function
//     let write_msg = Message::Write(String::from("Hello Rustaceans!"));
//     let change_color_msg = Message::ChangeColor(255, 165, 0); // Orange

//     println!("--- Processing Messages via Methods ---");
//     quit_msg.process();
//     move_msg.process();
//     write_msg.process();
//     change_color_msg.process();

//     println!("\nIs quit_msg a Quit message? {}", quit_msg.is_quit()); // true
//     println!("Is move_msg a Quit message? {}", move_msg.is_quit()); // false
// }
