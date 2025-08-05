// Primitive data types in Rust
// Int, Float, Bool, Char, Tuple, Array

// int types: i8, i16, i32, i64, i128, isize // signed integers
// uint types: u8, u16, u32, u64, u128, usize // unsigned integers
// The `isize` and `usize` types are used for indexing collections and are pointer-sized integers.
// The `i8` type is an 8-bit signed integer, `u8` is an 8-bit unsigned integer, and so on.
// The `i32` type is the default integer type in Rust, and it is a signed 32-bit integer.
// The range of `i32` is from -2,147,483,648 to 2,147,483,647, while `u32` ranges from 0 to 4,294,967,295.

// float types: f32, f64
// f32 is a 32-bit floating point, and f64 is a 64-bit floating point.
// The `f32` type is used for single-precision floating-point numbers, while `f64` is used for double-precision.
// The range of `f32` is approximately ±3.4 × 10^38, and for `f64`, it is approximately ±1.7 × 10^308.

// bool type: bool
// The `bool` type is used for boolean values, which can be either `true` or `false`.

// char type: char
// The `char` type in Rust represents a single Unicode scalar value, which means it can store any valid character from the Unicode standard, not just ASCII.
// Each `char` is 4 bytes in size and can represent characters like 'A', 'é', '中', or even emoji like '😊'.
// Example: let c: char = 'Z';

// tuple type: (T1, T2, ...)
// A tuple is a fixed-size collection of values of different types.
// Tuples can contain any number of elements, and each element can be of a different type.
// Example: let t: (i32, f64, char) = (42, 3.14, 'Z');
// The tuple type is defined with parentheses, and you can access its elements using dot notation, like `t.0`, `t.1`, etc.

// array type: [T; N]
// An array is a fixed-size collection of elements of the same type.
// Arrays in Rust are defined with square brackets, and the size of the array is part of its type.
// Example: let arr: [i32; 3] = [1, 2, 3];
// The array type is defined with square brackets, where `T` is the type of the elements and `N` is the number of elements.

// isize and usize are pointer-sized integers, used for indexing collections
// and are typically used when the size of the data structure is not known at compile time.
// The `usize` type is an unsigned integer type that is used for indexing and measuring the size of collections.
// The `isize` type is a signed integer type that is used for pointer arithmetic and can be used to represent the size of a collection in bytes.
mod deref_and_drop;
mod enums;
mod functions;
mod hash_maps;
mod oop;
mod ownership;
mod rust_arc;
mod rust_rc;
mod rust_refcell;
mod rust_thread;
mod rust_trait;
mod test_struct;
mod vectors;
fn main() {
    // Integer types
    let a: i32 = 10; // Signed 32-bit integer
    let b: u32 = 20; // Unsigned 32-bit integer

    // Floating-point types
    let x: f64 = 3.14; // 64-bit floating point
    let y: f32 = 2.71; // 32-bit floating point

    // Boolean type
    let is_active: bool = true;

    // Character type
    let letter: char = 'A';

    // Tuple type
    let tuple: (i32, f64, char) = (42, 3.14, 'Z');

    // Array type
    let array: [i32; 3] = [1, 2, 3];

    println!("Integer a: {}, b: {}", a, b);
    println!("Floating-point x: {}, y: {}", x, y);
    println!("Boolean is_active: {}", is_active);
    println!("Character letter: {}", letter);
    println!("Tuple values: {:?}", tuple);
    println!("Array values: {:?}", array);
}
