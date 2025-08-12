use std::fmt;

#[derive(Debug)]
pub enum ShopErr {
    ParseError,
    NotEnoughTokens,
}

#[derive(Debug)]

pub enum AuthError {
    UserNotFound,
    IncorrectPassword,
    UserExist,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::UserNotFound => write!(f, "User not found."),
            AuthError::IncorrectPassword => write!(f, "Incorrect password."),
            AuthError::UserExist => write!(f, "User exist Login"),
        }
    }
}

impl fmt::Display for ShopErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShopErr::ParseError => write!(f, "Invalid input. Please enter a number."),
            ShopErr::NotEnoughTokens => write!(f, "You do not have enough tokens."),
        }
    }
}
