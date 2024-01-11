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

## Requirements: 
R0: The Program shall accept no more than ten (10) Players.
R1: The Program shall accept no fewer than two (2) Players.
R2: The Program shall shuffle two (2) Card Decks together when there are more than five (5) Players.

