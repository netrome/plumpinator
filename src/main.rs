use plumpinator::card::PlayingCard;
use plumpinator::deck::Deck;

fn main() {
    let mut deck: Deck<PlayingCard> = Deck::new52();
    println!("Hello, world!");
    println!("{:?}", deck);
}
