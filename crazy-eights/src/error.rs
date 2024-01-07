use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DeckError {
    InvalidValue,
    InvalidSuit,
    DeckEmpty
}
impl Error for DeckError {}

impl fmt::Display for DeckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeckError::InvalidValue => write!(f, "Error: invalid Card Value"),
            DeckError::InvalidSuit => write!(f, "Error: invalid Card Suit"),
            DeckError::DeckEmpty => write!(f, "Error: deck empty"),
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    GameOver,
}
impl Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::GameOver => write!(f, "Error: Game is Over"),
        }
    }
}