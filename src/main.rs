use plumpinator::card::PlayingCard;
use plumpinator::deck::Deck;

fn main() {
    let mut deck: Deck<PlayingCard> = Deck::new52();
    println!("Hello, world!");
    println!("{:?}", deck);

    let my_hand = deck.take(8);
    let your_hand = deck.take(8);

    println!(".....");
    println!("My hand: {:?}", my_hand);
    println!("Your hand: {:?}", your_hand);
}
