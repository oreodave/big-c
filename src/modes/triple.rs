use crate::{card::Card, helper::ordered};

#[derive(Debug, Copy, Clone)]
pub struct Triple(Card, Card, Card);

impl Triple {
    /// Create a new triple utilising 3 cards: `c1`, `c2`, and `c3`.  Will
    /// return None iff a Triple cannot be constructed out of those 3 cards.
    ///
    /// NOTE: By construction, if a triple includes 1 Joker, then Triple::0 is
    /// that joker.  If a triple includes 2 jokers, then Triple::0 and Triple::1
    /// are those jokers.  This means Triple::2 will always be a valid playing
    /// card.
    pub fn new(c1: Card, c2: Card, c3: Card) -> Option<Triple> {
        let [c1, c2, c3] = ordered([c1, c2, c3]);

        match [c1, c2, c3].map(|c| c.rank()) {
            [None, None, Some(_)] => Some(Triple(c1, c2, c3)),
            [None, Some(r2), Some(r3)] if r2 == r3 => Some(Triple(c1, c2, c3)),
            [Some(r1), Some(r2), Some(r3)] if r1 == r2 && r2 == r3 => {
                Some(Triple(c1, c2, c3))
            }
            _ => None,
        }
    }

    fn high_pair(&self) -> Pair {
        Pair::new(self.1, self.2).unwrap()
    }

    fn count_jokers(&self) -> usize {
        [self.0, self.1, self.2]
            .iter()
            .filter(|c| c.is_joker())
            .count()
    }
}

use crate::helper::impl_cmp_eq_on_ord;
use std::cmp::Ordering;

impl Ord for Triple {
    fn cmp(&self, other: &Self) -> Ordering {
        let Triple(s1, s2, s3) = self;
        let Triple(o1, o2, o3) = other;

        // The most critical part of ordering is top card rank comparison.
        s3.rank()
            .cmp(&o3.rank())
            // if we have 2 triples, both with the same ranked high card, and
            // one has 2 jokers while the other doesn't => the 2 joker triple is
            // worse.
            .then_with(|| match (self.count_jokers(), other.count_jokers()) {
                (2, x) if x < 2 => Ordering::Less,
                (x, 2) if x < 2 => Ordering::Greater,
                _ => Ordering::Equal,
            })
            // then compare by the highest to lowest cards
            .then_with(|| s3.suit().cmp(&o3.suit()))
            .then_with(|| s2.cmp(o2))
            .then_with(|| s1.cmp(o1))
    }
}

impl_cmp_eq_on_ord!(Triple);

use crate::modes::{pair::Pair, Footstool, Hand};

impl Hand for Triple {
    fn is_proper(&self) -> bool {
        self.count_jokers() == 0
    }

    fn footstool(&self, other: &Self) -> Footstool {
        match self.cmp(other) {
            // There is no footstool if self is beaten by other.
            Ordering::Less => Footstool::None,
            // We can only full footstool if we have equivalent triples.
            Ordering::Equal => Footstool::Full,
            // Half footstools can only proc if the 2 high cards of each hand
            // footstool each other using Pair rules.
            Ordering::Greater => {
                // By construction, Triple::1 and Triple::2 should always make a
                // Pair so it's cool to unwrap.
                let [p1, p2] = [self, other].map(|x| x.high_pair());
                match p1.footstool(&p2) {
                    Footstool::Full => Footstool::Half,
                    _ => Footstool::None,
                }
            }
        }
    }
}

use std::fmt::{Display, Formatter, Result};

impl Display for Triple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Triple[{}, {}, {}]", self.0, self.1, self.2)
    }
}

use std::hash::{Hash, Hasher};

impl Hash for Triple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Pairs are just tuples lol.
        (self.0, self.1, self.2).hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::card::PlayingCard;

    use super::*;

    #[test]
    fn new() {
        let joker = Card::make_joker();

        // TEST: Cannot make a triple out of three jokers
        assert_eq!(
            Triple::new(joker, joker, joker),
            None,
            "Expected triple of 3 jokers to be None"
        );

        // TEST: Any card with two jokers is a triple
        for card in PlayingCard::iter_all(0).map(Card::PlayingCard) {
            let trip = Triple::new(card, joker, joker);
            assert!(
                trip.is_some(),
                "Expected ({card}, {joker}, {joker}) to make a triple"
            );
            let trip = trip.unwrap();
            assert_eq!(
                trip.2, card,
                "Expected the highest card of the triple ({}) to be the sole PlayingCard ({card})",
                trip.2
            );
        }

        todo!("Finish implementing");
    }
}
