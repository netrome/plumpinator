#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs::File;

use rocket_contrib::json::Json;

use plumpinator::card::PlayingCard;
use plumpinator::deck::Deck;


#[get("/deck/<name>")]
fn create_deck(name: String) -> Json<Deck<PlayingCard>> {
    let mut deck: Deck<PlayingCard> = Deck::new52();
    deck.shuffle();

    let file = File::create(format!("{}.json", name)).unwrap();
    serde_json::to_writer_pretty(file, &deck).unwrap();

    Json(deck)
}


#[get("/deck/<name>/<number>")]
fn get_hand(name: String, number: usize) -> Json<Deck<PlayingCard>> {
    let mut deck = if let Ok(file) = File::open(format!("{}.json", name))
    {
        serde_json::from_reader(file).unwrap()
    }
    else
    {
        Deck::new52()
    };

    let hand = deck.take(number);

    let file = File::create(format!("{}.json", name)).unwrap();
    serde_json::to_writer_pretty(file, &deck).unwrap();

    Json(hand)
}


fn main() {
    let mut deck: Deck<PlayingCard> = Deck::new52();

    deck.shuffle();
    let my_hand = deck.take(8);

    println!("{}", serde_json::to_string_pretty(&my_hand).unwrap());

    let file = File::create("hello.json").unwrap();
    serde_json::to_writer_pretty(file, &my_hand).unwrap();

    rocket::ignite().mount("/", routes![create_deck, get_hand]).launch();
}
