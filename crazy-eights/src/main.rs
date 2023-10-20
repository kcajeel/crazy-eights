struct Card {
    suit: String,
    number: i32,
}

fn main() {
    let mut deck: Vec<Card> = vec![];
    initialize_deck(&mut deck);
    println!("Listing cards: \n");
    print_deck(deck);
}

fn initialize_deck(deck: &mut Vec<Card>) {
    for i in 0..52 {
        print!("i/13: {}, i: {}\n", i / 13, i);
        deck.push(Card {
            number: if i % 13 != 0 { i % 13 } else { 13 },
            suit: if i / 13 < 1 {
                // 0
                "Hearts".to_string()
            } else if i / 13 > 0 && i / 13 < 2 {
                // 1
                "Diamonds".to_string()
            } else if i / 13 > 1 && i / 13 < 3 {
                // 2
                "Clubs".to_string()
            } else {
                // 3
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
}

fn print_deck(deck: Vec<Card>) {
    for i in deck.iter() {
        println!("Suit: {}, Number: {}", i.suit, i.number);
    }
}
