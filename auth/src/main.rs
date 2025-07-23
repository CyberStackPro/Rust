mod authentication;

use crate::authentication::user_management;

fn main() {
    println!("--- Application Start ---");

    user_management::authenticate_and_create(
        String::from("admin"),
        String::from("secure_pass"),
        String::from("admin@example.com"),
    );

    println!("--- Application End ---");

    // println!("Hello, world!");
}
