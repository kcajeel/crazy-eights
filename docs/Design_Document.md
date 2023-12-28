# Crazy Eight's Design Document

Contents:

[Setup](#setup)

[Gameplay](#gameplay)

[Program Design](#program-design)

[High Level Design](#high-level-design)

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

The program will be seaprated into various modules to divide the game into discrete parts. This should make development and maintennance easier, and the code will be separated into distinct files. The first modules to be defined are the card deck, the player structure, and the game itself. The main function will be used to launch the game and associated components.
Functions shall be designed to do one thing such that complex tasks are broken down into processes. Functions will aim to be relatively short (< 50 lines) to assist in this goal.
Named things shall be named according to their purpose, and units shall be appended to variable names where applicable. The goal of this is to create clear, readable code.
The program is designed to be self-documenting, i.e readable. This documentation exists to assist the developers in defining the program's design and to communicate the intended design to readers and potential contributors.

### High Level Design

This section goes over each file in the program and lists their contents. 

#### main.rs
-   The entrypoint for the program, the code in this file is light and relies on the underlying modules. 
-   `fn main()`: main function, used to call `game::run_game()`

#### deck.rs
-  Contains the functions and data structures necessary to construct a deck of cards. 
- `enum Value`: contains all possible Card values
- `enum Suit`: contains all possible Card suits
- `struct Card`: contains the information used in a playing card
- `fn new(&mut Vec<Card>)`: creates a standard 52-card deck with the Card struct
- `fn print(&Vec<Card>)`: prints the cards in the deck
- `fn shuffle(&mut Vec<Card>)`: shuffles the deck inplace

#### game.rs
- Contains the functions necessary to run the game logic
- `enum Game`: an enum with two variants: Running and Over, to signify the game's state.
- `fn initialize()`: initializes the game state by creating a Game struct with the appropriate number of players
- `fn deal_cards(&mut Vec<Player>)`: assigns a portion of the deck to each player in the list of players
- `fn play(&mut Game)`: calls each player's `take_turn` function in order until the game is won

#### player.rs
- Contains the data structures and functions for players to perform game actions
- `struct Player`: a representation of the player with a name and a `hand` field representing the cards in the player's hand
- `fn draw_card(&mut Vec<Card>)`: adds a card to the player's hand
- `fn play_card(&mut Vec<Card>)`: removes a given card from the player's hand
- `fn play_crazy_eight(&mut Vec<Card>, str)`: removes an eight from the player's hand and specifies the desired suit
- `fn take_turn(&mut Vec<Card>, &mut Vec<Card>)`: prompts the user associated with the player for input and allows them to draw or play a card from their hand

### Low Level Design

This section details each function, struct, and method contained within each module. It emphasizes implementation details such as the inner workings of functions and how modules and functions will relate to one another. 

#### main.rs
- `fn main(number_of_players: i32)` will call `game::Game::new(number_of_players)` to create a Game struct with the appropriate number of players. It will then call `game::run_game()` with the struct it created. 

#### deck.rs
- `enum Value`: will contain the variants for each value Name in the [Setup table](#setup)
- `enum Suit`: will contain variants for each suit, namely `Hearts`, `Diamonds`, `Spades`, and `Clubs`.
- `struct Card` will contain the following fields:
  - `value: Value`: contains the value of the card
  - `suit: Suit`: contains the suit of the card
  Implementations for Card: 
    - `new() -> Vec,Self>`: returns a Vec containing 52 Cards representing a standard card deck, henceforth referred to as a deck
    - `shuffle(&mut Vec<Card>)`: shuffles the deck in-place by cutting the it in half and inserting the cards back into the deck randomly multiple times
    - `print(&Vec<Card>)`: prints the contents of the deck

#### game.rs
- `enum Game` will contain the following variants:
  - `Running`: contains the following fields: 
    - `players: Vec<Player>`: contains a list of each player and their hand
    - `play_deck: Vec<Card>`:  holds the cards in the play deck, to be dealt to each player and added to the discard pile.
    - `discard_pile: Vec<Card>`: holds the cards in the discard pile
    - `suit_in_play: str`: contains a string of the current suit in play
  - `Over`: TODO: either contains no fields or contains the field of the winning player
  Implementations for Game: 
    - `fn new(number_of_players: i32, deck: Vec<Card>, pile: Vec,Card>) -> mut Self`: creates a new Game with the `players` field initialized to the `number_of_players` parameter
    - `fn initialize(&mut self)) -> mut Game`: Calls `deal_cards` with the fields of `self` and removes the top card of the `play_deck` and adds it to the `discard_pile`. `update_suit_in_play` is then called with the suit of the card in the discard pile to create the game's initial state. 
    - `fn deal_cards(players: &mut Vec<Player>, deck: &mut Vec<Card>)`: given the size of the `players` Vec, pops an appropriate ammount of cards (see [Setup](#setup)) from the `deck` into each player's hand in alternating order. 
    - `fn update_suit_in_play(new_suit: str)`: updates the `suit_in_play` field to the `new_suit` parameter.
    - `fn play(game: &mut Game)`: iterates through the `players` and calls `player::take_turn` on each player, passing the play deck and discard pile as parameters. 

#### player.rs
- `struct Player` will contain the following fields:
  - `name: String`: the name of the Player
  - `hand: Vec<Card>`: a list of cards in the player's posession
  Implementations for Player:
    - `fn draw_card(deck: &mut Vec<Card>, &mut self)`: Given a list of cards (the deck), the player pops the top of the deck and adds it to their hand.
    - `fn play_card(discard_pile: &mut Vec<Card>, card: Card), &mut self`: Given a list of cards (the discard pile) and a Card to be played, the card is added to the top of the discard pile and removed from the hand.
    - `fn play_crazy_eight(discard_pile: &mut Vec<Card>, eight: Card, new_suit: str, &mut self)`: Given a list of cards (the discard pile), a Card (the eight to be played), and a string (the suit to change to), the appropriate eight from the player's hand is added to the top of the discard pile. The Game method `update_suit_in_play` is then called with the suit to change to.
    - `fn can_play_card(&mut self, top_card: Card) -> bool`: checks if there is a card in the player's hand that matches the suit or value of the top card in the discard pile, returns true if there is a match.
    - `fn take_turn(deck: &mut Vec<Card>, discard_pile: &mut Vec<Card>, &mut self)`: Given the deck and discard pile as parameters, 
    calls `can_play_card` to determine if the user can play a card from their deck. If false, `draw_card` is called until `can_play_card` is true. Once a card can be played, the user is given a list of possible cards to play and asked to choose one. Once the user chooses a card, `play_card` or `play_crazy_eight` is called depending on their choice. If their choice is an eight, the user is also asked for the suit they would like to change to. 

---

## Future Plans

- A gui may be added once the game functions (defined above) are writen
- Once that is done, networking may be added to enable multiplayer features
- After that, this game will be re-written in C++ and then Java. It will be good experience to learn the differences between the languages, and it would be interesting to benchmark the performance of each program.
---

## References

- [Wikipedia](https://en.wikipedia.org/wiki/Crazy_Eights "Wikipedia")
- [Markdown Guide](https://www.markdownguide.org/extended-syntax/ "Markdown Guide")



[^1]: This range was chosen because the game cannot be played with less than two players, and the game would be confusing and time-consuming with more than ten players.

[^2]: The GUI would likely use a library crate, with art and graphics designed by project contributors. A CLI will likely be written for development and testing, and the GUI will be created once the game is fully tested. 

[1]: https://doc.rust-lang.org/nightly/rustc/platform-support.html