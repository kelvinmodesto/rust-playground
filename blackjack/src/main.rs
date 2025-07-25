use rand::{Rng, rng};
use std::io;

#[derive(Debug)]
struct Deck {
    cards: Vec<i32>,
}

impl Deck {
    fn build() -> Self {
        let mut cards = Vec::new();
        let num_decks = 4;

        for _ in 0..num_decks {
            for _ in 0..4 {
                for card in 1..=13 {
                    if card > 9 {
                        cards.push(10);
                    } else {
                        cards.push(card);
                    }
                }
            }
        }

        Deck { cards }
    }
}
fn main() {
    let idx = rng().random_range(0..13);
    let mut input = String::new();
    // let _ = io::stdin().read_line(&mut input);
    // let num: i32 = input.trim().parse().unwrap();
    let deck = Deck::build().cards;

    println!("You've typed: {:?}", deck);
}
