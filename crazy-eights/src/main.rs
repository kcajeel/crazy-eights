use rand::Rng;

#[derive(Clone)]
struct Card {
    suit: String,
    number: i32,
}

fn main() {
    let mut deck: Vec<Card> = vec![];
    initialize_deck(&mut deck);
    println!("\nListing cards: ");
    print_deck(&deck);

    for _i in 0..4 {
        shuffle(&mut deck);
    }
    println!("\nShuffled Deck: ");
    print_deck(&deck);
}

fn initialize_deck(deck: &mut Vec<Card>) {
    for i in 0..52 {
        deck.push(Card {
            // i%13 gives card number, if 0 then number = 13 (king)
            number: if i % 13 != 0 { i % 13 } else { 13 },
            // i/13 gives card suit (52/13=4)
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

fn print_deck(deck: &Vec<Card>) {
    for i in deck.iter() {
        println!("Suit: {}, Number: {}", i.suit, i.number);
    }
}

fn shuffle(deck: &mut Vec<Card>) {
    let mut deck_clone = deck.clone();
    let mut bottom_half = deck_clone.split_off(deck_clone.len() / 2);
    let mut top_half = deck_clone;
    let mut shuffled_deck: Vec<Card> = vec![];

    while !bottom_half.is_empty() || !top_half.is_empty() {
        let random_number = rand::thread_rng().gen::<i32>();

        if random_number % 2 == 0 && !bottom_half.is_empty() {

            if let Some(card) = bottom_half.pop() {
                shuffled_deck.push(card);
            }
        } else if !top_half.is_empty() {
            
            if let Some(card) = top_half.pop() {
                shuffled_deck.push(card);
            }
        }
    }
    *deck = shuffled_deck;
}
