use crate::card;

pub struct Deck<CardType: card::Card> {
    cards: Vec<CardType>,
}
