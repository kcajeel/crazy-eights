use std::fmt::Display;

use crate::error::DeckError;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, EnumIter, PartialEq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_string = match self {
            Value::Ace => "Ace",
            Value::Two => "Two",
            Value::Three => "Three",
            Value::Four => "Four",
            Value::Five => "Five",
            Value::Six => "Six",
            Value::Seven => "Seven",
            Value::Eight => "Eight",
            Value::Nine => "Nine",
            Value::Ten => "Ten",
            Value::Jack => "Jack",
            Value::Queen => "Queen",
            Value::King => "King",
        };
        write!(f, "{}", value_string)
    }
}
impl TryFrom<i32> for Value {
    type Error = DeckError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Value::Ace),
            2 => Ok(Value::Two),
            3 => Ok(Value::Three),
            4 => Ok(Value::Four),
            5 => Ok(Value::Five),
            6 => Ok(Value::Six),
            7 => Ok(Value::Seven),
            8 => Ok(Value::Eight),
            9 => Ok(Value::Nine),
            10 => Ok(Value::Ten),
            11 => Ok(Value::Jack),
            12 => Ok(Value::Queen),
            13 => Ok(Value::King),
            _ => Err(DeckError::InvalidValue), //implement error type
        }
    }
}
impl TryFrom<&str> for Value {
    type Error = DeckError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string.to_lowercase().as_str() {
            "ace" => Ok(Value::Ace),
            "two" => Ok(Value::Two),
            "three" => Ok(Value::Three),
            "four" => Ok(Value::Four),
            "five" => Ok(Value::Five),
            "six" => Ok(Value::Six),
            "seven" => Ok(Value::Seven),
            "eight" => Ok(Value::Eight),
            "nine" => Ok(Value::Nine),
            "ten" => Ok(Value::Ten),
            "jack" => Ok(Value::Jack),
            "queen" => Ok(Value::Queen),
            "king" => Ok(Value::King),
            _ => Err(DeckError::InvalidValue), //implement error type
        }
    }
}

#[derive(Clone, Copy, EnumIter, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit_string = match self {
            Suit::Clubs => "Clubs",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Spades => "Spades",
        };
        write!(f, "{}", suit_string)
    }
}
impl TryFrom<&str> for Suit {
    type Error = DeckError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string.to_lowercase().as_str() {
            "hearts" => Ok(Suit::Hearts),
            "diamonds" => Ok(Suit::Diamonds),
            "spades" => Ok(Suit::Spades),
            "clubs" => Ok(Suit::Clubs),
            _ => Err(DeckError::InvalidSuit),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn is_similar(some: &Card, other: &Card) -> bool {
        if some.suit.eq(&other.suit) || some.value.eq(&other.value) {
            return true;
        }
        return false;
    }

    pub fn print(&self) {
        println!("{} of {}", self.value, self.suit);
    }
}

pub fn new_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    for suit in Suit::iter() {
        for value in Value::iter() {
            deck.push(Card {
                value,
                suit: suit.clone(),
            });
        }
    }
    deck
}

pub fn shuffle_discard_pile(deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>) {
    if deck.is_empty() {
        if let Some(top_card) = discard_pile.pop() {
            deck.append(discard_pile);
            deck.shuffle(&mut rand::thread_rng());
            discard_pile.push(top_card);
        }
    }
}

pub fn print_deck(deck: &Vec<Card>) {
    for card in deck {
        Card::print(card);
    }
}
