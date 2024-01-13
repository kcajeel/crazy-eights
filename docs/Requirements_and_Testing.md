# Requirements and Testing Document

## About

This document will detail the program requirements to be used for testing and validation. The format for requiements shall be as follows: 

R[requirement number]: The [agent] shall [requirement]. 

Where the requirement number is an integer used to identify requirements in a concise way, the agent is one of the acceptable agents defined below, and the requirement is some behavior of the agent to be specified. Requirements shall be atomic, such that they only require one thing.

## Acceptable Agents

The following lists and defines the acceptable agents to be used in the agent field in requirement writing: 
- Program: The executable code
- User: The human interacting with the Program
- Player: The Player structure in the Program
- Card: The Card structure in the Program
- Card Deck: A group of fifty-two(52) Cards ranging in value from 1 (Ace) to 13 (King) and containing four Suits: Hearts, Diamonds, Spades, and Clubs
- Play Deck: A group of Cards that Players can draw from (not to be confused with the Discard Pile)
- Discard Pile: A group of Cards that Players play Cards into (not to be confused with the Play Deck)
- Hand: A group of cards held by a Player
- Top Card: The topmost Card in the Discard Pile
- Suit in Play: The Suit that Players must match with a Card from their Hand

## Requirements: 
R0: The Program shall accept no more than ten (10) Players.

R1: The Program shall accept no fewer than two (2) Players.

R2: The Program shall shuffle two (2) Card Decks together when there are more than five (5) Players.

R3: The Program shall shuffle one (1) Card Deck when there are five or fewer Players.

R4: The Program shall deal seven (7) Cards to each Player when there are two (2) Players.

R5: The Program shall deal five (5) cards to each Player when there are more than two (2) Players.

R6: The Discard Pile shall be empty before the Game is started.

R7: The Play Deck shall contain the cards specified in R2 or R3 before the Game is started.

R8: The Program shall remove the top Card from the Play Deck and insert that Card into the Discard Pile to start the Game after the Cards are dealt, as specified in R4 and R5.

R9: The Players shall take turns with only one (1) turn each until all other Players have taken a turn.

R10: The Program shall repeat R9 for each Player until a Player discards all of the Cards in their Hand.

R11: The Player shall discard a Card from their Hand if the Suit or Value of the Card matches the Top Card. 

R12: The Game shall allow the user to change the Suit in Play if they place a Card with the value Eight into the Discard Pile.
