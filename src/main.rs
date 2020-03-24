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

    let mut file = File::create(format!("{}.json", name)).unwrap();
    serde_json::to_writer_pretty(file, &deck).unwrap();

    Json(deck)
}


fn main() {
    let mut deck: Deck<PlayingCard> = Deck::new52();

    deck.shuffle();
    let my_hand = deck.take(8);

    println!("{}", serde_json::to_string_pretty(&my_hand).unwrap());

    let mut file = File::create("hello.json").unwrap();
    serde_json::to_writer_pretty(file, &my_hand).unwrap();

    rocket::ignite().mount("/", routes![create_deck]).launch();
}
