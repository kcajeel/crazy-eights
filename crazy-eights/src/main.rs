use rand::Rng;

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
    // shuffle cards
    // cut the deck, rearrange the halves
    // generate a random number and if even, add one half back, if odd add the other
    // repeat like 4 times

    let mut bottom_half = deck.split_off(deck.len() / 2);
    print!("size of bottom half: {}, size of top half: {}\n",bottom_half.len(),deck.len());
    let mut shuffled_deck: Vec<Card> = vec![];
    while !bottom_half.is_empty() && !deck.is_empty() {
        let random_number = rand::thread_rng().gen::<i32>();
        if random_number % 2 == 0 {
            // match bottom_half.pop() {
            //     Some(card) => shuffled_deck.push(card),
            //     None => 
            // }
            shuffled_deck.push(bottom_half.pop().unwrap());
        } else {
            // match deck.pop() {
            //     Some(card) => shuffled_deck.push(card),
            //     None => ,
            // }
            shuffled_deck.push(deck.pop().unwrap());
        }
    }
    println!("Shuffled deck size: {}, top half size: {}, bottom half size: {}",shuffled_deck.len(),deck.len(),bottom_half.len());
    *deck = shuffled_deck;
    println!("Final deck size: {}",deck.len());
}
