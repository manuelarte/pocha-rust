use std::{fmt, io};
use strum::IntoEnumIterator;
use crate::domain::card::{Card, Suite};
use crate::domain::card::Suite::OROS;
use rand::thread_rng;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaChaRng; // 0.1.1

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new(seed: [u8; 32]) -> Deck {
        let mut cards = Vec::new();
        for suite in Suite::iter() {
            for i in 1..8 {
                cards.insert(0, Card::new(suite, i));
            }
        }
        for suite in Suite::iter() {
            for i in 10..13 {
                cards.insert(0, Card::new(suite, i));
            }
        }
        let mut rng = ChaChaRng::from_seed(seed);
        cards.shuffle(&mut rng);
        return Deck{cards}
    }

    pub fn pop(&mut self, n: u8) -> Result<Vec<Card>, DeckError> {
        if self.cards.len() > usize::from(n) {
            let mut cards = Vec::new();
            for _ in 0..n {
                let card_option = self.cards.pop().unwrap();
                cards.push(card_option);
            }
            return Ok(cards);
        }
        return Err(DeckError::new("deck not big enough".to_string()))
    }

    pub fn len(&self) -> usize {
        return self.cards.len();
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.cards.len().to_string())
    }
}

pub struct DeckError {
    detail: String
}

impl DeckError {
    pub fn new(detail: String) -> Self{
        Self { detail }
    }
}