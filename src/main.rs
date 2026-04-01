mod card;
mod classifier;
mod hand;

use card::{make_decks, Card, Rank, Suit};
use classifier::classify;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::rng();
    let mut deck = make_decks(1);
    deck.shuffle(&mut rng);
    let hand = &mut deck[..5];

    // For testing specific examples.
    if false {
        let _hand = [
            Card::new(Rank::Nine, Suit::Diamond),
            Card::new(Rank::Ten, Suit::Diamond),
            Card::new(Rank::Jack, Suit::Club),
            Card::new(Rank::Queen, Suit::Spade),
            Card::new(Rank::Two, Suit::Spade),
        ];
    }
    hand.sort();

    for h in hand.iter() {
        print!("{}, ", h);
    }
    println!();

    let hand = classify(&hand);
    match hand {
        Some(hand) => println!("{}", hand),
        None => println!("Not a hand"),
    }
}
