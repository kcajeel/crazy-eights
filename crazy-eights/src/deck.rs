use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter)]
enum Value {
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
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(Value::Ace),
            2 => Some(Value::Two),
            3 => Some(Value::Three),
            4 => Some(Value::Four),
            5 => Some(Value::Five),
            6 => Some(Value::Six),
            7 => Some(Value::Seven),
            8 => Some(Value::Eight),
            9 => Some(Value::Nine),
            10 => Some(Value::Ten),
            11 => Some(Value::Jack),
            12 => Some(Value::Queen),
            13 => Some(Value::King),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, EnumIter)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    pub fn from_str(str: &str) -> Option<Suit> {
        match str {
            "Hearts" => Some(Suit::Hearts),
            "Diamonds" => Some(Suit::Diamonds),
            "Spades" => Some(Suit::Spades),
            "Clubs" => Some(Suit::Clubs),
            _ => None
        }
    }
}

#[derive(Clone)]
pub struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    pub fn new_deck() -> Vec<Self> {
        let mut deck:Vec<Card> = vec![];
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

    fn shuffle(deck: &mut Vec<Card>) {
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

    fn print(deck: &Vec<Card>) {
        for i in deck.iter() {
            println!("Suit: {:?}, Number: {:?}", i.suit, i.value);
        }
    }
}
