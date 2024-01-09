use game::Game;
use rand::seq::SliceRandom;

mod deck;
mod error;
mod game;
mod player;

fn prompt_user_for_number_of_players() -> i32 {
    println!("Please enter the number of players (2-10).");
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    match user_input.trim().parse::<i32>() {
        Ok(number_of_players) => {
            if number_of_players > 1 && number_of_players < 11 {
                number_of_players
            } else {
                prompt_user_for_number_of_players()
            }
        }
        Err(e) => {
            println!("{}", e);
            return prompt_user_for_number_of_players();
        }
    }
}
fn main() {
    let number_of_players = prompt_user_for_number_of_players();
    let mut deck = deck::new_deck();
    deck.shuffle(&mut rand::thread_rng());

    let mut game = Game::new(number_of_players, deck);
    game = game.initialize().unwrap().clone();
    game.play();
}
