use std::fmt;

#[derive(Debug)]
pub enum ShopErr {
    ParseError,
    NotEnoughTokens,
}

impl fmt::Display for ShopErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShopErr::ParseError => write!(f, "Invalid input. Please enter a number."),
            ShopErr::NotEnoughTokens => write!(f, "You do not have enough tokens."),
        }
    }
}
