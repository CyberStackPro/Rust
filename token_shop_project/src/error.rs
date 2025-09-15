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

pub enum InventoryError {
    InvalidItem,
    OutOfStock,
    ItemNotFound,
    DuplicateItem,
    InvalidPrice,
    InvalidStock,
    InvalidQuantity,
    NotEnoughTokens,
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

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::InvalidItem => write!(f, "Invalid Item"),
            InventoryError::OutOfStock => write!(f, "Item is out of stock."),
            InventoryError::ItemNotFound => write!(f, "Item not found."),
            InventoryError::DuplicateItem => write!(f, "Item already exists."),
            InventoryError::InvalidPrice => write!(f, "Invalid price."),
            InventoryError::InvalidStock => write!(f, "Invalid stock."),
            InventoryError::InvalidQuantity => write!(f, "Invalid Quantity."),
            InventoryError::NotEnoughTokens => write!(f, "Not enough tokens."),
        }
    }
}
