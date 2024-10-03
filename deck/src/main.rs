use rand::{random, rngs::OsRng, seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["hearts", "diamonds", "clubs", "spades"];
        let values = [
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }
    fn shuffle(&mut self) {
        // Shuffle the deck
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}
fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    //todo: error handling deal more cards than in deck
    deck.deal(49);

    println!("Here is your deck: {:#?}", deck);
}
