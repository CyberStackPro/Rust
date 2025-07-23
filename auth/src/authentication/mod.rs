pub fn login(username: &str, password: &str) -> bool {
    username == "admin" && password == "secure_pass"
}

fn logout(username: &str) {
    println!("Logging out {}", username);
}

// Declare submodule from file
pub mod user_management;
