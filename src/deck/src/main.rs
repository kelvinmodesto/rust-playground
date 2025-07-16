#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        Deck { cards: vec![] }
    }
    fn shuffle(&self) {}
    fn deal(&self) {}
}

fn main() {
    let deck = Deck::new();
    println!("Hello, world! {:?}", deck);
}
