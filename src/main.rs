// permit dead code when not using clippy
#![cfg_attr(not(clippy), allow(dead_code))]

use crate::game::deck::Deck;

mod card;
mod exactsizearr;
mod game;
mod helper;
mod modes;
mod zipcartesian;

fn main() {
    let mut rng = rand::rng();

    let (p1, p2, deck) = {
        let mut p1 = Deck::new_empty();
        let mut p2 = Deck::new_empty();
        let mut deck = Deck::new_full(2);

        deck.shuffle(&mut rng);
        deck.deal_tail(&mut p1, 13).unwrap();
        deck.deal_tail(&mut p2, 13).unwrap();
        (p1, p2, deck)
    };

    println!("{}\n{}\n{}", p1, p2, deck);
}
