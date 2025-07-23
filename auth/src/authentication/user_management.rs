// `super::login` = call `login` from parent module (authentication)
pub fn authenticate_and_create(username: String, password: String, email: String) {
    if super::login(&username, &password) {
        println!("Authentication successful for {}.", username);
        create_user(username, email);
    } else {
        println!("Authentication failed for {}.", username);
    }
}

// Function private to this file
fn delete_user(username: &str) {
    println!("Deleting user: {}", username);
}

// Public so other modules (like main.rs) can call it
pub fn create_user(username: String, email: String) {
    println!("Creating user: {} with email: {}", username, email);
}
