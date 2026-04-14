use crate::card::{Card, PlayingCard, Rank, Suit};

/// Excessively simple card iterator.
pub struct CardIterator(Card);

impl Iterator for CardIterator {
    type Item = Card;

    /// Generate the next card in the deck based on the current state of the
    /// CardIterator.  Iteration terminates at the 2 of Spades for the current
    /// deck.
    fn next(&mut self) -> Option<Card> {
        match self.0 {
            Card::Joker(_) => None,
            Card::PlayingCard(PlayingCard {
                rank: Rank::Two,
                suit: Suit::Spade,
                deck,
            }) => {
                self.0 = Card::Joker((deck + 1) * -1);
                None
            }
            Card::PlayingCard(pc) => {
                self.0 = Card::from(i64::from(pc) + 1);
                Some(self.0)
            }
        }
    }
}

impl DoubleEndedIterator for CardIterator {
    /// Generate the previous card in the deck based on the current state of the
    /// CardIterator.  Iteration terminates at the 3 of Diamonds for the current
    /// deck.
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.0 {
            Card::Joker(_) => None,
            Card::PlayingCard(PlayingCard {
                rank: Rank::Three,
                suit: Suit::Diamond,
                deck,
            }) => {
                self.0 = Card::Joker((deck + 1) * -1);
                None
            }
            Card::PlayingCard(pc) => {
                self.0 = Card::from(i64::from(pc) - 1);
                Some(self.0)
            }
        }
    }
}

impl Card {
    /// Create a CardIterator from the current card, moving the card in the
    /// process.
    pub fn into_iter(self) -> CardIterator {
        CardIterator(self)
    }
}
