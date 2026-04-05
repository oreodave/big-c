mod display;
mod hash;
mod impls;
mod numerics;
mod ord;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Rank {
    Three = 0,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Two,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Suit {
    Diamond = 0,
    Club,
    Heart,
    Spade,
}

#[derive(Eq, Debug, Clone, Copy)]
pub struct PlayingCard {
    pub deck: i64,
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Eq, Debug, Clone, Copy)]
pub enum Card {
    Joker(i64),
    PlayingCard(PlayingCard),
}

/** Given a sequence of Playing Cards, check if they are all of the same rank.
 */
pub fn all_same_rank(cards: &[PlayingCard]) -> bool {
    let rank = cards[0].rank;
    cards[1..].iter().all(|card| rank == card.rank)
}
