# Crazy Eight's Design Document

Contents:

Setup

Gameplay

Program Structure

High Level Design

Low Level Design

Future Plans

References

---

## Setup

As stated in the [README](../README.md "README"), this is a simulation of the card game Crazy Eights written in Rust. The game is played with a deck of cards, or two decks with more than five players. In either case, a deck consists of 52 cards with the jokers removed. A deck contains four suits: Hearts, Diamonds, Spades, and Clubs, which all contain the following values of cards:

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

To start the game, each player is dealt five cards from the shuffled deck (for two players, seven cards are dealt) and the top card of the remaining deck is overturned and placed face-up next to the deck, which will become the top card of the discard pile. The player to the left of the dealer is the first to start playing.

---

## Gameplay

The game is played in turns, where each player takes a turn in clockwise order.
On their turn, each player will look for a card in their hand that matches either the suit or value of the top card in the discard pile.
If a matching card is found, it is placed on top of the discard pile face-up and the next player starts their turn.
If the player does not have a matching card in their hand, they take cards from the deck until they find a match. If the deck runs out of cards, the top card is put aside and the discard pile is shuffled to become the new deck. The player will keep drawing until they find a match or an eight.
Eights of any suit can be played at any time on a player's turn, and playing an eight gives the player the option to change the suit in play. For example, if Player 1 has no cards that match the top of the discard pile but their hand contains an eight, they can play the eight and change the suit to one that they have in their hand.
The game is played until a player discards all of the cards in their hand and becomes the winner. In games of more than two people, the remaining players can decide to keep playing until another player discards their entire hand, claiming second place.

---

## Project Scope

This program will simulate the Crazy Eights card game with all of the gameplay elements described above. The game's functions will be implemented, and the game will be fully playable with 2-10 players[^note]. It is not within the scope of the project to include networking for multiple players, but it may be added in the future. The game will be accessible either through a CLI or GUI[^note], which will be decided once the core game functionality is completed. Because this program is written in Rust, it should be runnable on [every platform Rust supports][1]. 

---

## Program Design

The program will be seaprated into various modules to divide the game into discrete parts. This should make development and maintennance easier, and the game should be more loosely coupled as a result. The first modules to be defined are the card deck, the player entities, and the game itself. The main function will be used to launch the game and associated components.
Functions shall be designed to do one thing such that complex tasks are broken down into processes. Functions will aim to be relatively short (< 50 lines) to assist in this goal.
Named things shall be named according to their purpose, and units shall be appended to variable names where applicable. The goal of this is to create clear, readable code.
The program is designed to be self-documenting, i.e readable. This documentation exists to assist the developers in defining the program's design and to communicate the intended design to readers and potential contributors.

### High Level Design

### Low Level Design

---

## Future Plans

---

## References

[Wikipedia](https://en.wikipedia.org/wiki/Crazy_Eights "Wikipedia")

---

[^note]: This range was chosen because the game cannot be played with less than two players, and the game would be confusing and time-consuming with more than ten players.

[^note]: The GUI would likely use a library crate, with art and graphics designed by project contributors. A CLI will likely be written for development and testing, and the GUI will be created once the game is fully tested. 

[1]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
