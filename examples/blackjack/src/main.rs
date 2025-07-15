use rand::{Rng, rng};
use std::io;

// let mut deck = Vec<str>::new();
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn build() -> Self {}
}
fn main() {
    let idx = rng().random_range(0..13);
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let num: i32 = input.trim().parse().unwrap();

    println!("You've typed: {}", num);
}
