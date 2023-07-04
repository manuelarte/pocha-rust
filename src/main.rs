use crate::domain::deck::Deck;

mod domain;

fn main() {
    println!("Hello, world!");

    let seed = [0; 32];
    let mut deck = Deck::new(seed);
    let hand = deck.pop(4);
    println!("{}", deck)
}
