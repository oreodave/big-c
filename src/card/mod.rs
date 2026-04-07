mod default;
mod display;
mod hash;
mod impls;
mod numerics;
mod ord;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Debug, Copy, Clone)]
pub enum Rank {
    #[default]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Debug, Copy, Clone)]
pub enum Suit {
    #[default]
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
