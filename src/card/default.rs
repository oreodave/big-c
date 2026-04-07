use crate::card::{Card, PlayingCard, Rank, Suit};

impl Default for PlayingCard {
    fn default() -> Self {
        Self::new(0, Rank::default(), Suit::default())
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::PlayingCard(PlayingCard::default())
    }
}
