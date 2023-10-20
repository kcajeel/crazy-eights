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
            suit: match i / 13 {
                0 => "Hearts".to_string(),
                1 => "Diamonds".to_string(),
                2 => "Clubs".to_string(),
                3 => "Spades".to_string(),
                _ => "Joker???".to_string(),
            },
        })
    }
}

fn print_deck(deck: Vec<Card>) {
    for i in deck.iter() {
        println!("Suit: {}, Number: {}", i.suit, i.number);
    }
}
