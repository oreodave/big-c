use crate::{card::Card, helper::ordered};

#[derive(Eq, Debug, Copy, Clone)]
pub struct Triple(Card, Card, Card);

impl Triple {
    pub fn new(c1: Card, c2: Card, c3: Card) -> Option<Triple> {
        let [c1, c2, c3] = ordered([c1, c2, c3]);

        match [c1, c2, c3].map(|c| c.rank()) {
            // Two Jokers + any PlayingCard
            [None, None, Some(_)] => Some(Triple(c1, c2, c3)),

            // One Joker + two PlayingCards of the same rank
            [None, Some(r2), Some(r3)] if r2 == r3 => Some(Triple(c1, c2, c3)),

            // Three PlayingCards of the same rank
            [Some(r1), Some(r2), Some(r3)] if r1 == r2 && r2 == r3 => {
                Some(Triple(c1, c2, c3))
            }

            _ => None,
        }
    }
}

use std::cmp::Ordering;

impl Ord for Triple {
    fn cmp(&self, other: &Self) -> Ordering {
        /*
        Like pairs, we'll do a high-to-low comparison.  Since we're dealing with
        3 items we'll need to compute potentially 3 comparisons.
        */
        let Triple(s1, s2, s3) = self;
        let Triple(o1, o2, o3) = other;

        let cmp = s3.cmp(o3);
        if cmp != Ordering::Equal {
            cmp
        } else {
            let cmp = s2.cmp(o2);
            if cmp != Ordering::Equal {
                cmp
            } else {
                s1.cmp(o1)
            }
        }
    }
}

impl PartialEq for Triple {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Triple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
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
            assert_eq!(trip.2, card, "Expected the highest card of the triple ({}) to be the sole PlayingCard ({card})", trip.2);
        }

        todo!("Finish implementing");
    }
}
