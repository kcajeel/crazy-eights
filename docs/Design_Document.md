# Crazy Eight's Design Document

Contents:

[Setup](#setup)

[Gameplay](#gameplay)

[Program Design](#program-design)

[Low Level Design](#low-level-design)

[Future Plans](#future-plans)

[References](#references)

---

## Setup

As stated in the [README](../README.md "README"), this is a simulation of the card game Crazy Eights written in Rust. The game is played with either a deck of cards, or two decks with more than five players. In either case, a deck consists of 52 cards with the jokers removed. A deck contains four suits: Hearts, Diamonds, Spades, and Clubs, which all contain the following values of cards:

| Value | Name |
| :---: | :---: |
|   1   |  Ace  |
|   2   |  Two  |
|   3   | Three |
|   4   | Four |
|   5   | Five |
|   6   |  Six  |
|   7   | Seven |
|   8   | Eight |
|   9   | Nine |
|  10  |  Ten  |
|  11  | Jack |
|  12  | Queen |
|  13  | King |

The deck(s) are shuffled before the game begins so that the cards are mostly out of order.

To start the game, each player is dealt five cards from the shuffled deck (for two players, seven cards are dealt) and the top card of the remaining deck is overturned and placed face-up next to the deck, which will become the discard pile. The player to the left of the dealer is the first to start playing.

---

## Gameplay

The game is played in turns, where each player takes a turn in clockwise order.
On their turn, a player will look for a card in their hand that matches either the suit or value of the top card in the discard pile.
If a matching card is found, it is placed on top of the discard pile face-up and the next player starts their turn.
If the player does not have a matching card in their hand, they take cards from the deck until they find a match. If the deck runs out of cards, the top card of the discard pile is put aside and the rest of the discard pile is shuffled to become the new deck. The player will keep drawing until they find a match or an eight.
Eights of any suit can be played at any time on a player's turn. Playing an eight gives the player the option to change the suit in play. For example, if a player has no cards that match the top of the discard pile but their hand contains an eight, they can play the eight and change the suit to one that they have in their hand.
The game is played until a player discards all of the cards in their hand and becomes the winner. In games of more than two people, the remaining players can decide to keep playing until another player discards their entire hand, claiming second place.

---

## Project Scope

This program will simulate the Crazy Eights card game with all of the gameplay elements described above. The game's functions will be implemented, and the game will be fully playable with 2-10 players[^1]. It is not currently within the scope of the project to include networking for multiple human players, but it may be added in the future. There will be automated players for cases where there are not enough human players for a game. The game will be accessible either through a CLI or GUI[^2], which will be decided once the core game functionality is completed. Because this program is written in Rust, it should be runnable on [every platform Rust supports][1]. 

---

## Program Design


The program will be separated into various modules to divide the game into discrete parts. This should make development and maintenance easier, and the code will be more organized this way. The first modules to be defined are the card deck, the player structure, the game itself, and finally a module for defining error types. The main function will be used to launch the game and associated components.
Functions shall be designed to do one thing such that complex tasks are broken down into procedures. Functions will aim to be relatively short (< 20 lines) to assist in this goal. Some things such as user input and display code will inherently take more lines than algorithmic code, so this rule may have some exceptions in the actual implementation.
Named things shall be named according to their purpose, and units shall be appended to variable names where applicable. The goal of this is to create clear, readable code.
The program is designed to be self-documenting, i.e. readable. This documentation exists to assist the developers in defining the program's design and to communicate the intended design to readers and potential contributors.

## Low Level Design

This section details each function, struct, and method contained within each module. It emphasizes implementation details such as the inner workings of functions and how modules and functions will relate to one another. 

#### main.rs
- `fn promp_user_for_number_of_players() -> i32`: prompts the user for a number of players and returns the number
- `fn main()` will call `pompt_user_for_number_of_players` to get the number of players. Then calls `Game::new()` to create a Game struct with the appropriate number of players. It will then call `run_game()` with the struct it created. 

#### deck.rs
- `pub enum Value`: will contain the variants for each value Name in the [Setup table](#setup)
- `pub enum Suit`: will contain variants for each suit, namely `Hearts`, `Diamonds`, `Spades`, and `Clubs`.
- `pub struct Card` will contain the following fields:
  - `pub value: Value`: contains the value of the card
  - `pub suit: Suit`: contains the suit of the card

  Implementations for Card: 
    - `pub fn is_similar(some: &Card, other: &Card) -> bool`: returns true if either the Suit or Value fields of `some` match `other`.
    - `pub fn print(&self)`: prints the value and suit of the card

  Deck functions:
  - `pub fn new() -> Vec<Self>`: returns a Vec containing 52 Cards representing a standard card deck, henceforth referred to as a deck
  - `pub fn shuffle_discard_pile(deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>)`: adds all of the cards except for the top card from the discard pile to the deck, then shuffles the deck.
  - `pub fn print_deck(&Vec<Card>)`: calls `Card::print` on the contents of the deck

#### player.rs
- `pub struct Player` will contain the following fields:
  - `pub name: String`: the name of the Player
  - `pub hand: Vec<Card>`: a list of cards in the player's possession
  
  Implementations for Player:
    - `pub fn new(name: String, hand: Option<Vec<Card>>) -> Self`: Given a name and optionally a hand, returns a new Player.
    - `fn draw_card(hand: &mut Vec<Card>, deck: &mut Vec<Card>)`: Given a list of cards (the deck), the player pops the top of the deck and adds it to their hand.
    - `fn get_playable_cards(hand: &Vec<Card>, top_card: Card) -> Vec<Card>`: given a player's hand and the top card of the discard pile, returns a list of all the cards in the player's hand that can be played using `Card::is_similar`.
    - `fn prompt_user_for_card(cards: Vec<Card>) -> Card`: prompts the user to choose a card to play from a list of possible cards and returns the chosen card.
    - `fn prompt_user_for_suit() -> Suit`: prompts the user to choose a suit to change the discard pile to and returns the chosen suit.
    - `fn change_suit_in_play(mut old_suit: &Suit, new_suit: &Suit)`: assigns the value of `new_suit` to `old_suit`.
    - `fn play_card(hand: &mut Vec<Card>, discard_pile: &mut Vec<Card>, card: Card, suit_in_play: &mut Suit)`: Given the discard pile and a Card to be played, the card is added to the top of the discard pile and removed from the hand. The current suit in play is changed with `change_suit_in_play`. If the card's value is an Eight, then `prompt_user_for_suit` is called and the suit in play is changed appropriately.
    - `fn take_turn(hand: &mut Vec<Card>, deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>, suit_in_play: &mut Suit)`: Given the deck and discard pile as parameters, calls `get_playable_cards` to determine if the user can play any cards from their deck. If the list is empty, `draw_card` is called until the list of playable cards is not empty. Once a card can be played, `prompt_user_for_card` is called with a list of the possible cards. Once the user chooses a card, `play_card` is called.

#### game.rs
- `pub enum Game` will contain the following variants:
  - `Running`: contains the following fields: 
    - `players: Vec<Player>`: contains a list of each player and their hand
    - `deck: Vec<Card>`:  holds the cards in the play deck, to be dealt to each player and added to the discard pile.
    - `discard_pile: Vec<Card>`: holds the cards in the discard pile
    - `suit_in_play: str`: contains a string of the current suit in play
  - `Over`: TODO: either contains no fields or contains the field of the winning player

   Implementations for Game: 
    - `fn new(number_of_players: i32, deck: Vec<Card>, pile: Vec,Card>) -> mut Self`: creates a new Game with the `players` field initialized to the `number_of_players` parameter. If the number of players is less than two or greater than ten, it panics.
    - `fn get_player_names(number_of_players: i32) -> Vec<String>`: given the number of players in the game, prompts the user for a name for each player and returns the list of names.
    - `fn initialize_players(number_of_players: i32, names: Vec<String>) -> Vec<Player>`: given the number of players and list of names, initializes the appropriate number of Players to the list of names with empty `hand` fields and returns a list of the players.
    - `fn deal_cards(players: &mut Vec<Player>, deck: &mut Vec<Card>)`: given the size of the `players` Vec, pops an appropriate amount of cards (see [Setup](#setup)) from the `deck` into each player's hand in alternating order. 
    - `fn initialize_discard_pile(deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>) -> Result<Card, DeckError>`: adds the top card of the deck to the discard pile. Returns a `DeckEmpty` error if `deck.pop()` returns None, and returns the top card otherwise.
    - `fn initialize(&mut self) -> Result<&mut Self, Box<dyn Error>>`: If the `self` reference is an instance of `Game::Running`, calls `deal_cards` with the fields of `self` and calls `initialize_discard_pile`, proaogating any errors to the caller. `update_suit_in_play` is then called with the suit of the card in the discard pile to create the game's initial state. If the `self` reference is an instance of `Game::Over`, a `GameOver` error is returned.
    - `fn end_game(&mut self)`: sets `self` to an instance of `Game::Over`.
    - `fn play(&mut self)`: iterates through the `players` and calls `player::take_turn` on each player, passing the play deck and discard pile as parameters until the game ends, where `end_game` is called.

#### error.rs
- `pub enum DeckError` contains the following variants: 
  - `InvalidValue`: returned by `Value.try_from()`
  - `InvalidSuit`: returned by `Suit.try_from()`
  - `DeckEmpty`: returned in `Game::initialize_discard_pile()` if the deck is empty.
- `pub enum GameError` contains the following variants:
  - `GameOver`: returned by `Game::initialize()` when the game is over and nothing can be initialized.
---

## Future Plans

- A tui will be added once the game functions (defined above) are written
- Once that is done, networking may be added to enable multiplayer features
---

## References

- [Wikipedia](https://en.wikipedia.org/wiki/Crazy_Eights "Wikipedia")
- [Markdown Guide](https://www.markdownguide.org/extended-syntax/ "Markdown Guide")



[^1]: This range was chosen because the game cannot be played with less than two players, and the game would be confusing and time-consuming with more than ten players.

[^2]: The GUI would likely use a library crate, with art and graphics designed by project contributors. A CLI will likely be written for development and testing, and the GUI will be created once the game is fully tested. 

[1]: https://doc.rust-lang.org/nightly/rustc/platform-support.html