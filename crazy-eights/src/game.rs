use crate::{
    deck::{Card, Suit},
    player::{Player, self},
};

#[derive(PartialEq)]
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
    fn new(number_of_players: i32, deck: Vec<Card>) -> Self {
        if is_number_of_players_valid(number_of_players) {
            Game::Running {
                players: Self::initialize_players(
                    number_of_players,
                    Self::get_player_names(number_of_players),
                ),
                deck,
                discard_pile: vec![],
                suit_in_play: Suit::Clubs,
            }
        } else {
            Game::Over
        }
    }

    fn get_player_names(number_of_players: i32) -> Vec<String> {
        println!("There are {} players in this game. ", number_of_players);
        let mut player_names = vec![];
        for i in 1..number_of_players {
            println!("Please enter the name for player {}.", i);
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).expect("Failed to read input");
            println!("You entered: {}", user_input);
            player_names.push(user_input);
        }
        player_names
    }

    fn initialize_players(number_of_players: i32, mut names: Vec<String>) -> Vec<Player> {
        let mut players = Vec::with_capacity(number_of_players as usize);

        for _i in 0..number_of_players {
            players.push(Player::new(
                match names.pop() {
                    Some(name) => name,
                    None => "".to_string(),
                },
                vec![],
            ));
        }
        players
    }

    fn deal_cards(players: &mut Vec<Player>, deck: &mut Vec<Card>) {
        let cards_per_player = if players.len() == 2 { 7 } else { 5 };

        for _i in 0..cards_per_player {
            for player in &mut *players {
                if let Some(card) = deck.pop() {
                    player.hand.push(card);
                }
            }
        }
    }

    pub fn initialize(&mut self) {
        if let Game::Running {
            players,
            deck,
            discard_pile,
            suit_in_play,
        } = self
        {
            Self::deal_cards(players, deck);
            if let Ok(card) = initialize_discard_pile(deck, discard_pile) {
                *suit_in_play = card.suit;
            }
        }
    }

    pub fn play(&mut self) {
        if let Game::Running {
            players,
            deck,
            discard_pile,
            suit_in_play,
        } = self
        {
            for player in players {
                while !player.hand.is_empty() {
                    Player::take_turn(&mut player.hand, deck, discard_pile, suit_in_play);
                }
            }
        }
        self.end_game();
    }

    pub fn end_game(&mut self) {
        *self = Game::Over;
    }
}

fn initialize_discard_pile(
    deck: &mut Vec<Card>,
    discard_pile: &mut Vec<Card>,
) -> Result<Card, String> {
    if let Some(card) = deck.pop() {
        discard_pile.push(card);
        return Ok(card);
    } else {
        return Err("deck empty".to_string());
    }
}

fn is_number_of_players_valid(number_of_players: i32) -> bool {
    if number_of_players >= 2 && number_of_players <= 10 {
        return true;
    }
    false
}
