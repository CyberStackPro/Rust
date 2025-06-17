#![allow(warnings)]
fn main() {
    // let age: u16 = 18;
    // if age >= 18 {
    //     println!("You can drive a car!");
    // } else {
    //     println!("You can't drive a car");
    // }

    let number = 5;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 2,3,4")
    }
}
