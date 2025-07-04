// Functions In Rust
// Functions in Rust are defined using the `fn` keyword, followed by the function name, parameters, and a return type.
// Functions can take parameters, return values, and can be called from other parts of the code.
// Functions can also be defined with default parameters, and they can be generic, allowing them to work with different types.
// Functions can be defined at the top level or inside other functions, and they can be used to organize code into reusable components.
// Functions can also be used to implement traits, which are a way to define shared behavior across different types.

fn main() {
    hello_world();
    tell_height(4294967295);
    human_id("Hello world", 30, 34.5);

    let x = {
        let price = 100;
        let discount = 20;
        price - discount // This is an expression that evaluates to 80
        // (OPTIONAL) if the last line in a block is an expression, it can be used as a value
    };

    print!("Value of x: {}", x);
    let sum = add(5, 10);
    println!("\nSum of 5 and 10 is: {}", sum);

    // the BMI fn
    calculate_bmi(70.0, 1.75); // weight in kg, height in meters 
    let str = String::from("Hello, Rust!");
    let length = calculate_str(&str);
    println!("The length of the string '{}' is: {}", str, length);
}

fn add(x: i32, y: i32) -> i32 {
    x + y // This is an expression that evaluates to the sum of x and y
}
fn hello_world() {
    println!("Hello world")
}

fn tell_height(height: u32) {
    println!("Your height is: {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("Name: {}, Age: {}, Height: {} cm", name, age, height);
}

// Expression and Statement
// In Rust, an expression is a piece of code that evaluates to a value, while a statement is a line of code that performs an action but does not return a value.
// Expressions can be used in various contexts, such as in function arguments, variable assignments, or control flow statements.
// For example, the following code contains both an expression and a statement:
// let x = 5; // This is a statement that assigns the value 5 to the variable x
// let y = x + 2; // This is an expression that evaluates to 7 and assigns it to y
// In this case, `x + 2` is an expression that evaluates to a value, while `let y = ...` is a statement that assigns that value to the variable `y`.age

// BMI=height(kg)/height(m)^2
fn bmi(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}
fn bmi_category(bmi: f32) -> &'static str {
    if bmi < 18.5 {
        "Underweight"
    } else if bmi < 24.9 {
        "Normal weight"
    } else if bmi < 29.9 {
        "Overweight"
    } else {
        "Obesity"
    }
}
fn calculate_bmi(weight: f32, height: f32) {
    let bmi_value = bmi(weight, height);
    let category = bmi_category(bmi_value);
    println!(
        "Your BMI is: {:.2}, which falls into the category: {}",
        bmi_value, category
    );
}

fn calculate_str(str: &str) -> usize {
    str.len() // This is an expression that evaluates to the length of the string
}

// Analogy:
// Borrowing in Rust is similar to passing props in React: both are about sharing data safely without direct modification.

// 2. Dangling Pointers
// A dangling pointer is a pointer that refers to memory that has already been freed or is otherwise invalid. Using it can cause crashes or unpredictable behavior.

// Rust prevents dangling pointers by making sure references can’t outlive the data they point to.

// 3. Data Races
// A data race happens when:

// Two or more threads access the same memory at the same time,
// At least one thread is writing,
// There’s no synchronization.
// This can lead to bugs that are hard to find.

// Rust prevents data races at compile time by enforcing strict rules about how data is accessed and shared between threads.

// Summary Table:

// Concept	What it is	How Rust/React handles it
// Borrowing/Props	Sharing data without modifying it	Rust: references, React: props
// Dangling Pointer	Pointer to invalid memory	Rust: prevents at compile time
// Data Race	Unsafe concurrent memory access	Rust: prevents at compile time
