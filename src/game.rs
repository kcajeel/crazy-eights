use std::error::Error;

use crate::{
    deck::{Card, Suit},
    error::{DeckError, GameError},
    player::Player,
};

#[derive(Clone, PartialEq)]
pub enum Game {
    Running {
        players: Vec<Player>,
        deck: Vec<Card>,
        discard_pile: Vec<Card>,
        suit_in_play: Suit,
    },
    Over,
}

impl Game {
    pub fn new(number_of_players: i32, deck: Vec<Card>) -> Self {
        Game::Running {
            players: Self::initialize_players(
                number_of_players,
                Self::get_player_names(number_of_players),
            ),
            deck,
            discard_pile: vec![],
            suit_in_play: Suit::Clubs,
        }
    }

    fn get_player_names(number_of_players: i32) -> Vec<String> {
        println!("There are {} players in this game. ", number_of_players);
        let mut player_names = vec![];
        for i in 1..=number_of_players {
            println!("Please enter the name for player {}.", i);
            let mut user_input = String::new();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read input");
            println!("You entered: {}", user_input);
            player_names.push(user_input);
        }
        player_names
    }

    fn initialize_players(number_of_players: i32, mut names: Vec<String>) -> Vec<Player> {
        let mut players = Vec::with_capacity(number_of_players as usize);

        for i in 0..number_of_players {
            players.push(Player::new(
                names
                    .pop()
                    .expect(&format!("Empty player name for player {}", i)),
                vec![],
            ));
        }
        players
    }

    fn deal_cards(players: &mut Vec<Player>, deck: &mut Vec<Card>) {
        let cards_per_player = if players.len() == 2 { 7 } else { 5 };

        for i in 0..cards_per_player {
            for player in &mut *players {
                player.hand.push(deck.pop().expect(&format!(
                    "deck empty at card {} for player {}",
                    i, player.name
                )));
            }
        }
    }

    fn initialize_discard_pile(
        deck: &mut Vec<Card>,
        discard_pile: &mut Vec<Card>,
    ) -> Result<Card, DeckError> {
        if let Some(card) = deck.pop() {
            discard_pile.push(card);
            return Ok(card);
        } else {
            return Err(DeckError::DeckEmpty);
        }
    }

    pub fn initialize(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        if let Game::Running {
            players,
            deck,
            discard_pile,
            suit_in_play,
        } = self
        {
            Self::deal_cards(players, deck);
            let card = Self::initialize_discard_pile(deck, discard_pile)?;
            *suit_in_play = card.suit;
            return Ok(self);
        }
        Err(Box::new(GameError::GameOver))
    }

    pub fn end_game(&mut self) {
        *self = Game::Over;
    }

    pub fn play(&mut self) {
        if let Game::Running {
            players,
            deck,
            discard_pile,
            suit_in_play,
        } = self
        {
            'game: loop {
                for player in &mut *players {
                    println!("\nPlayer {}'s turn", player.name.trim());
                    Player::take_turn(&mut player.hand, deck, discard_pile, suit_in_play);
                    if player.hand.is_empty() {
                        println!("\nPlayer {} wins! ", player.name.trim());
                        break 'game;
                    }
                }
            }
        }
        self.end_game();
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_num_cards_two_players() {}
    #[test]
    fn test_num_cards_gt_two_players() {}
    #[test]
    fn test_discard_pile_empty() {}

    #[test]
    fn test_play_deck_state() {}

    
}
