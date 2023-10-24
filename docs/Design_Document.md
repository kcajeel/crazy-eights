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

| Value | Name  |
| :---: | :---: |
|   1   |  Ace  |
|   2   |  Two  |
|   3   | Three |
|   4   | Four  |
|   5   | Five  |
|   6   |  Six  |
|   7   | Seven |
|   8   | Eight |
|   9   | Nine  |
|  10   |  Ten  |
|  11   | Jack  |
|  12   | Queen |
|  13   | King  |

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

## Program Structure: High Level Design

## Proram Structure: Low Level Design

## Future Plans

## References
[Wikipedia](https://en.wikipedia.org/wiki/Crazy_Eights "Wikipedia")
