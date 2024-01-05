use rand::{Rng, seq::SliceRandom};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
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
impl Value {
    pub fn from_int(value: i32) -> Result<Self, String> {
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
            _ => Err("invalid card value".to_string()),    //implement error type
        }
    }

    pub fn from_string(string: &str) -> Result<Self, String> {
            match string.to_lowercase().as_str() {
                "one" => Ok(Value::Ace),
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
                _ => Err("invalid card Value".to_string()),    //implement error type
            }
        
    }
}


#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
impl Suit {
    pub fn from_string(string: &str) -> Result<Self, String> {
        match string.to_lowercase().as_str() {
            "hearts" => Ok(Suit::Hearts),
            "diamonds" => Ok(Suit::Diamonds),
            "spades" => Ok(Suit::Spades),
            "clubs" => Ok(Suit::Clubs),
            _ => Err("invalid card Suit".to_string()),    //implement error type
        }
    
}
}


#[derive(Clone, Copy, PartialEq)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn is_similar(some_card: &Card, top_card: &Card) -> bool {
        if some_card.suit.eq(&top_card.suit) || some_card.value.eq(&top_card.value) {
            return true;
        }
        return false;
    }

    pub fn print(&self) {
        println!("{:?} of {:?}", self.value, self.suit);
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

//I might delete this and use the Rand shuffle because it seems more efficient
pub fn shuffle(deck: &mut Vec<Card>) {
    let mut deck_clone = deck.clone();
    let mut bottom_half = deck_clone.split_off(deck_clone.len() / 2);
    let mut top_half = deck_clone;
    let mut shuffled_deck: Vec<Card> = vec![];

    while !bottom_half.is_empty() || !top_half.is_empty() {
        let random_number = rand::thread_rng().gen::<i32>();

        if random_number % 2 == 0 && !bottom_half.is_empty() {
            if let Some(card) = bottom_half.pop() {
                shuffled_deck.push(card);
            }
        } else if !top_half.is_empty() {
            if let Some(card) = top_half.pop() {
                shuffled_deck.push(card);
            }
        }
    }
    *deck = shuffled_deck;
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
    for i in deck.iter() {
        println!("Suit: {:?}, Number: {:?}", i.suit, i.value);
    }
}
