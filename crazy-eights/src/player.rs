use crate::
    deck::{Card, Suit, Value}
;

pub struct Player {
    pub name: String,
    pub hand: Option<Vec<Card>>,
}

impl Player {
    fn new(name: String, hand: Option<Vec<Card>>) -> Self {
        Self { name, hand }
    }

    fn draw_card(hand: &mut Vec<Card>, deck: &mut Vec<Card>) {
        match deck.pop() {
            Some(card) => hand.push(card),
            None => panic!(), //TODO: implement DeckEmptyError
        }
    }

    //TODO: consider moving this function to another scope
    //It doesn't need to be a method of Card, but it should be associated
    fn is_card_playable(some_card: &Card, top_card: &Card) -> bool {
        if some_card.suit.eq(&top_card.suit) || some_card.value.eq(&top_card.value) {
            return true;
        }
        return false;
    }

    fn get_playable_cards(hand: &Vec<Card>, top_card: &Card) -> Vec<Card> {
        let mut playable_cards = vec![];
        for card in hand {
            if Self::is_card_playable(card, top_card) {
                playable_cards.push(*card);
            }
        }
        playable_cards
    }

    fn prompt_user_for_card(cards: Vec<Card>) -> Card {
        //TODO: implement user input
    }

    fn play_card(hand: &mut Vec<Card>, discard_pile: &mut Vec<Card>, card: &Card, suit_in_play: &mut Suit) {
        if hand.contains(card) {
            discard_pile.push(*card); //add card to discard pile
            hand.retain(|&x| x != *card); //delete card from hand
            
            let mut new_suit_in_play = card.suit;   //get new suit in play
            if card.value == Value::Eight {
                new_suit_in_play = Self::prompt_user_for_suit();    //change the suit if crazy eight
            }
            Self::change_suit_in_play(&suit_in_play, &new_suit_in_play);
        }
    }

    fn prompt_user_for_suit() -> Suit {
        //TODO: implement user input
    }

    fn change_suit_in_play(mut old_suit: &Suit, new_suit: &Suit) {
        old_suit = new_suit;
    }

    fn take_turn(hand: &mut Vec<Card>, deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>, suit_in_play: &mut Suit) {
        if let Some(top_card) = discard_pile.get(0) {
            let playable_cards = Self::get_playable_cards(hand, top_card);
            
            while playable_cards.is_empty() {
                if deck.is_empty() {
                    //shuffle all discard_pile except the top
                }
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
