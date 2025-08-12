use chrono::Utc;

use crate::error::AuthError;

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>,
    pub tokens: i32,
}

// pub fn authenticate_and_create(
//     username: &str,
//     email: &str,
//     password: &str,
//     db: &mut Vec<User>,
// ) -> Result<User, AuthError> {
//     match login(email, password, db) {
//         Ok(user) => {
//             println!(
//                 "[USER LOGIN] Authentication successful for {}",
//                 user.username
//             );
//             Ok(user.clone())
//         }
//         Err(AuthError::UserNotFound) => {
//             println!("[NEW USER] User not found. Creating new user...");
//             create_user(
//                 username.to_string(),
//                 email.to_string(),
//                 password.to_string(),
//                 db,
//             )?;
//             Ok(db.last().unwrap().clone())
//         }
//         Err(e) => {
//             println!("[AUTH ERROR] Authentication failed: {:?}", e);
//             Err(e)
//         }
//     }
// }

pub fn login<'a>(email: &str, password: &str, db: &'a Vec<User>) -> Result<&'a User, AuthError> {
    let user = db
        .iter()
        .find(|user| user.email == email)
        .ok_or(AuthError::UserNotFound)?;

    if user.password == password {
        Ok(user)
    } else {
        Err(AuthError::IncorrectPassword)
    }
}

// pub fn login(email: &str, password: &str, db: &mut Vec<User>) -> Result<User, AuthError> {
//     match db.iter().find(|user| user.email == email) {
//         Some(user) if user.password == password => Ok(user.clone()),
//         Some(_) => Err(AuthError::IncorrectPassword),
//         None => Err(AuthError::UserNotFound),
//     }
// }

pub fn register(
    username: String,
    email: String,
    password: String,
    db: &mut Vec<User>,
) -> Result<User, AuthError> {
    if db.iter().any(|user| user.email == email) {
        return Err(AuthError::UserExist);
    }

    let new_user = User {
        username,
        email,
        password,
        created_at: Utc::now(),
        tokens: 0,
    };

    db.push(new_user.clone());

    Ok(new_user)
}
