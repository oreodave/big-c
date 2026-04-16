// permit dead code when not using clippy
#![cfg_attr(not(clippy), allow(dead_code))]

use crate::{
    game::{deck::Deck, playerbuilder::PlayerBuilder},
    modes::{item::Item, pair::Pair},
};

mod card;
mod exactsizearr;
mod game;
mod helper;
mod modes;
mod zipcartesian;

fn find_first_pair(deck: &Deck) -> Pair {
    for i in 0..deck.len() - 1 {
        let cards = deck.get(&[i, i + 1]).unwrap();
        match Item::parse(&cards) {
            Ok(Item::Pair(pair)) => return pair,
            _ => continue,
        }
    }
    // FIXME: There is totally a way this happens, however unlikely.  If we have
    // no jokers and at most one instance of each rank.  But I've got my fingers
    // in my ears.
    panic!("Shouldn't happen mate....");
}

fn main() {
    let mut rng = rand::rng();

    let (p1, p2, mut players) = {
        let mut players = PlayerBuilder::new();
        let p1 = players.add();
        let p2 = players.add();
        (p1, p2, players.into_fixed())
    };

    // Deal out some cards based for our two players.
    {
        let mut deck = Deck::new_full(2);
        deck.shuffle(&mut rng);
        // Since we know Deck::new_full(2) must have 108 cards, dealing 26 cards
        // off the tail should be just fine.
        deck.deal_tail(&mut players[p1], 13).unwrap();
        deck.deal_tail(&mut players[p2], 13).unwrap();
    };

    // Let's try to parse a pair off player 1 and player 2.
    let [p1pair, p2pair] = [p1, p2].map(|x| &players[x]).map(find_first_pair);
    println!(
        "{} vs {}\n{}",
        p1pair,
        p2pair,
        match p1pair.cmp(&p2pair) {
            std::cmp::Ordering::Less => "p2 won!",
            std::cmp::Ordering::Greater => "p1 won!",
            std::cmp::Ordering::Equal => "p1 and p2 lost",
        }
    );

    println!("{}\n{}", players[p1], players[p2]);
}
