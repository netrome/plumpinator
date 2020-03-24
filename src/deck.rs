use std::cmp;
use std::iter::FromIterator;

use rand::seq::SliceRandom;

use crate::card;

#[derive(Debug)]
pub struct Deck<CardType: card::Card> {
    cards: Vec<CardType>,
}

impl<CardType: card::Card> Deck<CardType> {
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn new52() -> Self {
        Self::from_iter(0..52)
    }

    pub fn take(&mut self, n: usize) -> Self {
        let mid = cmp::min(n, self.cards.len());

        Self::from_iter(self.cards.drain(..mid))
    }
}

impl<CardType: card::Card> std::iter::FromIterator<CardType> for Deck<CardType> {
    fn from_iter<I: IntoIterator<Item = CardType>>(iter: I) -> Self {
        Deck {
            cards: iter.into_iter().collect(),
        }
    }
}

impl<CardType: card::Card> std::iter::FromIterator<u8> for Deck<CardType> {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        Deck {
            cards: iter.into_iter().map(CardType::from).collect(),
        }
    }
}
