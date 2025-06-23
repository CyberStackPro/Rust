// Error handing in Rust
// Error handling in Rust is a crucial aspect of writing robust and reliable code.
// Rust provides a strong type system and a set of conventions for handling errors, which helps prevent many common programming mistakes.
// In Rust, errors are typically handled using the `Result` and `Option` types.
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;
use std::result::Result;
use std::str::FromStr;
// Custom error type
#[derive(Debug)]
struct MyError {
    details: String,
}
impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.details)
    }
}
impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}
// Function that returns a Result type
fn read_file_content<P: AsRef<Path>>(path: P) -> Result<String, MyError> {
    let mut file = File::open(path).map_err(|e| MyError::new(&e.to_string()))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| MyError::new(&e.to_string()))?;
    Ok(content)
}
// Function that parses a string into an integer
fn parse_integer(s: &str) -> Result<i32, ParseIntError> {
    i32::from_str(s)
}
// Function that demonstrates error handling
fn handle_errors() -> Result<(), Box<dyn Error>> {
    // Attempt to read a file
    let content = read_file_content("example.txt")?;
    println!("File content: {}", content);

    // Attempt to parse an integer
    let number = parse_integer("42")?;
    println!("Parsed number: {}", number);

    Ok(())
}
// Main function to run the error handling example
fn main() {
    match handle_errors() {
        Ok(_) => println!("All operations completed successfully."),
        Err(e) => eprintln!("An error occurred: {}", e),
    }

    // Example of handling a specific IO error
    let result = File::open("non_existent_file.txt");
    match result {
        Ok(_) => println!("File opened successfully."),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => eprintln!("File not found: {}", e),
            _ => eprintln!("An error occurred while opening the file: {}", e),
        },
    }
}
// This code demonstrates how to handle errors in Rust using custom error types, the Result type, and pattern matching.
// It includes functions for reading file content and parsing integers, along with error handling mechanisms.
// The main function runs the error handling example and prints appropriate messages based on the success or failure of operations.
// The code also shows how to handle specific IO errors using the ErrorKind enum.
// This approach ensures that errors are handled gracefully, providing clear feedback to the user or developer.
