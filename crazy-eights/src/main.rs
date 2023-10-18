struct Card {
    suit: String,
    number: i32,
}

impl Card {}

fn main() {
    let deck: Vec<Card> = vec![];
    initialize_deck(deck);
    println!("Crazy Eights Game!");
}

fn initialize_deck(mut deck: Vec<Card>) -> Vec<Card> {
    for i in 1..53 {
        deck.push(Card {
            number: if i % 13 != 0 { i % 13 } else { 13 },
            suit: if i / 13 < 1 {
                "Hearts".to_string()
            } else if i / 13 > 1 && i / 13 < 2 {
                "Diamonds".to_string()
            } else if i / 13 > 2 && i / 13 < 3 {
                "Clubs".to_string()
            } else {
                "Spades".to_string()
            },
        })
    }
    /*
    suit
    i/13:
    < 1: hearts
    > 1 && < 2: diamonds
    > 2 && < 3: clubs
    > 3 && < 4: spades
    */

    // card number = i%13
    // if i%13 == 0, card number is 13 (King)
    deck
}
