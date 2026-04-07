use crate::{
    card::{Card, PlayingCard},
    helper::ordered,
};

#[derive(Eq, Debug, Copy, Clone)]
pub struct Triple(Card, Card, Card);

impl Triple {
    fn new(c1: Card, c2: Card, c3: Card) -> Option<Triple> {
        let [c1, c2, c3] = ordered([c1, c2, c3]);

        match (c1, c2, c3) {
            (Card::Joker(_), Card::Joker(_), Card::Joker(_)) => None,
            // NOTE: c3 should not be a joker now.
            (Card::Joker(_), Card::Joker(_), _) => Some(Triple(c1, c2, c3)),
            (
                Card::Joker(_),
                Card::PlayingCard(PlayingCard { rank: r1, .. }),
                Card::PlayingCard(PlayingCard { rank: r2, .. }),
            ) if r1 == r2 => Some(Triple(c1, c2, c3)),

            (
                Card::PlayingCard(PlayingCard { rank: r1, .. }),
                Card::PlayingCard(PlayingCard { rank: r2, .. }),
                Card::PlayingCard(PlayingCard { rank: r3, .. }),
            ) if r1 == r2 && r2 == r3 => Some(Triple(c1, c2, c3)),

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

