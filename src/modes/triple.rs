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
    use crate::{
        card::{PlayingCard, Rank},
        zipcartesian::ZipCartesianExt,
    };

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

        for card in PlayingCard::iter_all(0).map(Card::PlayingCard) {
            let trip = Triple::new(card, joker, joker);
            // TEST: Any card with two jokers is a triple
            assert_ne!(
                trip, None,
                "Expected ({card}, {joker}, {joker}) to make a triple"
            );
            let trip = trip.unwrap();

            // TEST: Triples formed with 2 jokers are improper.
            assert!(trip.is_improper(), "Expected {trip} to be improper");

            // TEST: Triples with 2 jokers should have a playing card for
            // Triple::2.
            assert_eq!(
                trip.2, card,
                "Expected the highest card of the triple ({}) to be the sole PlayingCard ({card})",
                trip.2
            );
        }

        // Iterate over all pairs of cards with similar ranks
        for (c1, c2) in
            Rank::iter_all().flat_map(|r| r.cards().zip_cartesian(r.cards()))
        {
            let trip = Triple::new(c1, c2, joker);
            // TEST: Any two similar rank cards with 1 joker are a
            // Triple.
            assert_ne!(
                trip, None,
                "Expected ({c1}, {c2}, Joker) to make a Triple"
            );

            let trip = trip.unwrap();

            // TEST: Triples formed with 1 joker are improper.
            assert!(trip.is_improper(), "Expected {trip} to be improper");

            // TEST: 1 joker triples have Triple::0 as the joker.
            assert!(
                matches!(trip.0, Card::Joker(_)),
                "Expected {} to be a joker",
                trip.0
            );

            let [c1, c2] = ordered([c1, c2]);

            // TEST: Expect Triple::1 and Triple::2 to follow ordering
            // of c1 and c2.
            assert_eq!(
                [trip.1, trip.2],
                [c1, c2],
                "Expected {} = min({c1}, {c2}) and {} = max({c1}, {c2})",
                trip.1,
                trip.2,
            );
        }

        // Iterate over all pairs of cards with differing ranks
        for (c1, c2) in Rank::iter_all()
            .flat_map(|r1| {
                Rank::iter_all()
                    .filter(move |&r2| r2 != r1)
                    .map(move |r2| (r1, r2))
            })
            .flat_map(|(r1, r2)| r1.cards().zip_cartesian(r2.cards()))
        {
            // TEST: Cannot make a triple out of 1 joker and two different rank
            // cards
            let trip = Triple::new(c1, c2, joker);
            assert_eq!(
                trip, None,
                "Expected ({c1}, {c2}, {joker}) to never make a triple."
            );
        }

        // Iterate over all triples of cards (regardless of rank)
        for (c1, (c2, c3)) in PlayingCard::iter_all(0).zip_cartesian(
            PlayingCard::iter_all(0).zip_cartesian(PlayingCard::iter_all(0)),
        ) {
            let [c1, c2, c3] = [c1, c2, c3].map(Card::PlayingCard);
            let trip = Triple::new(c1, c2, c3);

            // TEST: Any 3 playing cards make a triple iff they match in
            // rank
            if !(c1.rank() == c2.rank() && c2.rank() == c3.rank()) {
                assert_eq!(
                    trip, None,
                    "Expected {c1}, {c2}, {c3} to never make a Triple."
                );
                continue;
            } else {
                assert_ne!(
                    trip, None,
                    "Expected {c1}, {c2}, {c3} to make a Triple."
                );
            }

            let trip = trip.unwrap();
            // TEST: Triples formed of 3 playing cards are proper.
            assert!(trip.is_proper(), "Expected {trip} to be proper");

            let [c1, c2, c3] = ordered([c1, c2, c3]);

            // TEST: If a triple is formed of 3 playing cards, they are
            // ordered s.t. Triple::2 > Triple::1 > Triple::0.
            assert_eq!(
                [trip.0, trip.1, trip.2],
                [c1, c2, c3],
                "Expected cards of {} to match ordered cards [{}, {}, {}]",
                trip,
                c1,
                c2,
                c3
            );
        }
    }
    }
}
