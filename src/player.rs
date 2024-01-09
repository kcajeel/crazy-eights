use std::io;

use crate::deck::{self, Card, Suit, Value};

#[derive(Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(name: String, hand: Vec<Card>) -> Self {
        Self { name, hand }
    }

    pub fn draw_card(hand: &mut Vec<Card>, deck: &mut Vec<Card>) {
        println!("Drawing from the deck. ");
        if let Some(card) = deck.pop() {
            hand.push(card);
            println!("You drew the ");
            card.print();
        } else {
            panic!("Error: deck empty");
        }
    }

    fn get_playable_cards(hand: &Vec<Card>, top_card: &Card, suit_in_play: &mut Suit) -> Vec<Card> {
        let mut playable_cards = vec![];

        for card in hand {
            if top_card.suit != *suit_in_play {
                if card.suit == *suit_in_play {
                    playable_cards.push(*card);
                }
            } else {
                if Card::is_similar(card, top_card) {
                    playable_cards.push(*card);
                }
            }
        }
        playable_cards
    }

    fn prompt_user_for_card(cards: Vec<Card>) -> Card {
        println!("You can play the following cards: \n");
        deck::print_deck(&cards);
        println!("Which card would you like to play? ");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line");
        print!("You played \"{}\".\n", user_input.trim());

        let input_vec: Vec<&str> = user_input.split(' ').collect();

        if input_vec.len() != 3 {
            return Self::prompt_user_for_card(cards);
        }
        // unwrap used below because input_vec.len > 3 so .get(0..2) should return Some
        Card {
            value: match Value::try_from(input_vec.get(0).unwrap().trim()) {
                Ok(val) => val,
                Err(_e) => {
                    println!("error parsing value \"{}\"", input_vec.get(0).unwrap());
                    cards.get(0).unwrap().value
                }
            },
            suit: match Suit::try_from(input_vec.get(2).unwrap().trim()) {
                Ok(suit) => suit,
                Err(_e) => {
                    println!("error parsing suit \"{}\"", input_vec.get(2).unwrap());
                    cards.get(0).unwrap().suit
                }
            },
        }
    }

    fn prompt_user_for_suit() -> Suit {
        println!("You have played a Crazy Eight. Which suit would you like to select?");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("could not read input");
        println!("You selected: {}", user_input.trim());

        match Suit::try_from(user_input.trim()) {
            Ok(suit) => suit,
            Err(_e) => {
                print!("Invalid suit. Defaulting to Clubs.\n");
                Suit::Clubs
            }
        }
    }

    fn change_suit_in_play(_old_suit: &mut Suit, new_suit: Suit) {
        *_old_suit = new_suit;
        println!("new suit: {}", _old_suit);
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
            println!(
                "suit in play changed from {} to {}",
                suit_in_play, new_suit_in_play
            );
            Self::change_suit_in_play(suit_in_play, new_suit_in_play);
        } else {
            println!("{} of {} not found in your hand. ", card.value, card.suit);
        }
    }

    pub fn take_turn(
        hand: &mut Vec<Card>,
        deck: &mut Vec<Card>,
        discard_pile: &mut Vec<Card>,
        suit_in_play: &mut Suit,
    ) {
        let binding = discard_pile.clone();
        let top_card = binding
            .get(discard_pile.len() - 1)
            .expect("discard pile empty");
        let mut playable_cards = Self::get_playable_cards(hand, &top_card.clone(), suit_in_play);

        print!(
            "You have {} cards in your hand. Card to match: ",
            hand.len()
        );
        top_card.print();
        while playable_cards.is_empty() {
            print!("No playable cards. ");
            if deck.is_empty() {
                println!("The deck is empty, shuffling the discard pile into the deck.");
                deck::shuffle_discard_pile(deck, discard_pile);
            }
            Self::draw_card(hand, deck);
            playable_cards = Self::get_playable_cards(hand, top_card, suit_in_play);
        }
        Self::play_card(
            hand,
            discard_pile,
            &Self::prompt_user_for_card(playable_cards),
            suit_in_play,
        );
    }
}
