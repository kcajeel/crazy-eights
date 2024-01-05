use std::io;

use crate::deck::{Card, Suit, Value};

#[derive(PartialEq)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(name: String, hand: Vec<Card>) -> Self {
        Self { name, hand }
    }

    pub fn draw_card(hand: &mut Vec<Card>, deck: &mut Vec<Card>) {
        match deck.pop() {
            Some(card) => hand.push(card),
            None => panic!(), //TODO: implement DeckEmptyError
        }
    }

    fn get_playable_cards(hand: &Vec<Card>, top_card: &Card) -> Vec<Card> {
        let mut playable_cards = vec![];
        for card in hand {
            if Card::is_similar(card, top_card) {
                playable_cards.push(*card);
            }
        }
        playable_cards
    }

    fn prompt_user_for_card(cards: Vec<Card>) -> Card {
        println!("You can play the following cards: \n");

        for card in cards {
            Card::print(&card);
        }

        //This implementation is just for testing, I plan on using the "tui" and "crossterm" crates for a "real" UI. Hence, no error checking here.
        println!("Which card would you like to play? ");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line");
        println!("You played {}.", user_input);

        let input_vec: Vec<&str> = user_input.split(' ').collect();

        let card_value = if let Some(value) = input_vec.get(0) {
            match Value::from_string(*value) {
                Ok(val) => val,
                Err(_error) => Value::Ace,
            }
        } else {
            Value::Ace
        };

        let card_suit = if let Some(suit) = input_vec.get(2) {
            match Suit::from_string(*suit) {
                Ok(suit) => suit,
                Err(_error) => Suit::Clubs,
            }
        } else {
            Suit::Clubs
        };
        Card {
            value: card_value,
            suit: card_suit,
        }
    }

    fn play_card(
        hand: &mut Vec<Card>,
        discard_pile: &mut Vec<Card>,
        card: &Card,
        suit_in_play: &mut Suit,
    ) {
        if hand.contains(card) {
            discard_pile.push(*card); //add card to discard pile
            hand.retain(|&x| x != *card); //delete card from hand

            let mut new_suit_in_play = card.suit; //get new suit in play
            if card.value == Value::Eight {
                new_suit_in_play = Self::prompt_user_for_suit(); //change the suit if crazy eight
            }
            Self::change_suit_in_play(&suit_in_play, &new_suit_in_play);
        }
    }

    fn prompt_user_for_suit() -> Suit {
        println!("You have played a Crazy Eight. Which suit would you like to select?");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("could not read input");
        println!("You selected: {}", user_input);

        match Suit::from_string(&user_input.trim()) {
            Ok(suit) => suit,
            Err(_error) => Suit::Clubs,
        }
    }

    fn change_suit_in_play<'a>(mut _old_suit: &'a Suit, new_suit: &'a Suit) {
        _old_suit = new_suit;
    }

    pub fn take_turn(
        hand: &mut Vec<Card>,
        deck: &mut Vec<Card>,
        discard_pile: &mut Vec<Card>,
        suit_in_play: &mut Suit,
    ) {
        if let Some(top_card) = discard_pile.pop() {
            let playable_cards = Self::get_playable_cards(hand, &top_card);

            while playable_cards.is_empty() {
                if deck.is_empty() {}
                Self::draw_card(hand, deck);
            }

            Self::play_card(
                hand,
                discard_pile,
                &Self::prompt_user_for_card(playable_cards),
                suit_in_play,
            );
        }
    }
}
