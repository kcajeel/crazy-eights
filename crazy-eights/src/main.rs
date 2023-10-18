
enum Card {
    Suit,
    Number,
}

fn main() {
    let mut deck: Vec<Card> = vec![];
    initialize_deck(deck);
    println!("Crazy Eights Game!");
}

fn initialize_deck(deck: Vec<Card>) -> Vec<Card> {
    for i in 1..52 {
        // card number = i%13
            // if i%13 == 0, card number is 13 (King)
        // suit = match floor(52/i)
            // 4 -> hearts
            // 3 -> diamonds
            // 2 -> clubs
            // 1 -> spades
    }
    deck
}