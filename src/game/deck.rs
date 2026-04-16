use std::fmt::{Display, Formatter};

use crate::card::Card;
use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
/// A Deck of Cards - essentially a container of cards.
pub struct Deck(Vec<Card>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// Reasons for why an operation may fail.
pub enum Reason {
    NotSorted,
    OutOfBounds(usize),
}

impl Deck {
    pub fn new_empty() -> Self {
        Self(Vec::new())
    }

    /// Create a new deck composed of `n` of decks of cards (using
    /// `Card::iter_all`).  Guaranteed to have 54`n` cards by construction.
    pub fn new_full(n: usize) -> Self {
        assert!(n > 0);
        Self(Card::iter_all(n as i64).collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn shuffle<T: Rng>(&mut self, rng: &mut T) {
        self.0.shuffle(rng);
    }

    /// Sort cards within the deck by the ordering implementation for Cards.
    /// See [[file:../card/ord.rs::impl Ord for Card {]].
    pub fn sort(&mut self) {
        self.0.sort();
    }

    /// Sort cards within the deck by suit in ascending order.
    pub fn sort_by_suit(&mut self) {
        // Sort by suit then rank
        self.0.sort_by(|x, y| {
            x.suit()
                .cmp(&y.suit())
                .then_with(|| x.rank().cmp(&y.rank()))
        })
    }

    /// Add a set of cards to the end of `self`.
    pub fn add(&mut self, cards: &[Card]) {
        self.0.extend_from_slice(cards);
    }

    /// Get a set of cards at indices `indices` as a vector.
    /// Returns Err if any index is out of bounds.
    pub fn get(&self, indices: &[usize]) -> Result<Vec<Card>, Reason> {
        let mut collector = Vec::with_capacity(indices.len());
        for &ind in indices {
            if ind >= self.len() {
                return Err(Reason::OutOfBounds(ind));
            }
            collector.push(self.0[ind]);
        }
        Ok(collector)
    }

    /// Remove cards at indices `indices` from `self`.  Order is not preserved.
    /// Returns Err if `indices` is not sorted in ascending order or if any
    /// index is out of bounds.
    pub fn remove(&mut self, indices: &[usize]) -> Result<(), Reason> {
        if !indices.is_sorted() {
            Err(Reason::NotSorted)
        } else if let Some(index) = indices.iter().find(|&&x| x >= self.len()) {
            Err(Reason::OutOfBounds(*index))
        } else {
            for &index in indices.iter().rev() {
                self.0.swap_remove(index);
            }
            Ok(())
        }
    }

    /// Remove `n` cards from the end of `self`, and append them to the deck
    /// `other`.
    /// Returns Err if the number of cards requested exceed the size of the
    /// current deck.
    pub fn deal_tail(
        &mut self,
        other: &mut Self,
        n: usize,
    ) -> Result<(), Reason> {
        if n > self.0.len() {
            Err(Reason::OutOfBounds(n))
        } else {
            let mut tail = self.0.split_off(self.len() - n);
            tail.sort();
            other.0.append(&mut tail);
            Ok(())
        }
    }

    /// Remove cards at indices `indices` from `self` and append them onto the
    /// deck `other`.  Order is not preserved.
    /// Returns Err if `indices` are not sorted in ascending order or if any
    /// indices are out of bounds.
    pub fn deal_any(
        &mut self,
        other: &mut Self,
        indices: &[usize],
    ) -> Result<(), Reason> {
        let mut removed_cards = self.get(indices)?;
        self.remove(indices)?;
        other.0.append(&mut removed_cards);
        Ok(())
    }

    /// Given two indices (`a` and `b`) in the deck, swap the cards in `self`.
    /// Returns Err if either `a` or `b` are out of bounds.
    pub fn swap(&mut self, a: usize, b: usize) -> Result<(), Reason> {
        if a >= self.len() {
            Err(Reason::OutOfBounds(a))
        } else if b >= self.len() {
            Err(Reason::OutOfBounds(b))
        } else {
            self.0.swap(a, b);
            Ok(())
        }
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, c) in self.0.iter().enumerate() {
            write!(f, "{}", c)?;
            if i < self.0.len() - 1 {
                write!(f, ", ")?
            }
        }
        write!(f, "}}")
    }
}
