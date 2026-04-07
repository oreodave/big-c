use crate::card::{Card, PlayingCard, Rank, Suit};
use std::hash::{Hash, Hasher};

impl Hash for Rank {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as i64).hash(state);
    }
}

impl Hash for Suit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as i64).hash(state);
    }
}

impl Hash for PlayingCard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // NOTE: We're using the i64 conversion of card for the hash since that
        // should generate unique numbers per card.
        i64::from(*self).hash(state);
    }
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // NOTE: We're using the i64 conversion of card for the hash since that
        // should generate unique numbers per card.
        i64::from(*self).hash(state);
    }
}
