use crate::card::{Card, PlayingCard};
use std::cmp::Ordering;

impl Ord for PlayingCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.abs().cmp(&other.abs())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::PlayingCard(c1), Self::PlayingCard(c2)) => c1.cmp(c2),
            // Jokers should not really care about internal ordering.
            (Self::Joker(_), Self::Joker(_)) => Ordering::Equal,
            // Jokers are the lowest possible card so any Playing Cards are
            // better than them.
            (Self::Joker(_), _) => Ordering::Less,
            (_, Self::Joker(_)) => Ordering::Greater,
        }
    }
}

impl PartialEq for PlayingCard {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for PlayingCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
