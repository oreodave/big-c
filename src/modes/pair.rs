use crate::card::{Card, PlayingCard};
use crate::helper::ordered;

#[derive(Eq, Debug, Copy, Clone)]
pub struct Pair(pub Card, pub Card);

impl Pair {
    /** Create a new pair utilising two cards, `c1` and `c2`.  Will return None
    if a Pair cannot be constructed out of the two cards.

    NOTE: By construction, if the Pair includes a Joker, that Joker will be the
    first member of the pair.
     */
    fn new(c1: Card, c2: Card) -> Option<Pair> {
        // Order the cards.  This means if xor(c1 is joker, c2 is joker) c1 will
        // be that joker.
        let (c1, c2) = ordered(c1, c2);

        match (c1, c2) {
            // Can't be a pair if you got two jokers homie.
            (Card::Joker(_), Card::Joker(_)) => None,

            // NOTE: c2 cannot be a joker because of prev condition.  If you've
            // got a joker you're automatically a pair.
            (Card::Joker(_), _) => Some(Pair(c1, c2)),

            // NOTE: c1 and c2 cannot be jokers.  In which case, check their
            // ranks are equivalent.
            (
                Card::PlayingCard(PlayingCard { rank: r1, .. }),
                Card::PlayingCard(PlayingCard { rank: r2, .. }),
            ) => (r1 == r2).then_some(Pair(c1, c2)),

            // Not necessary since the previous patterns technically cover all
            // cases.  But I love the compiler too much to tell them... 💔
            _ => None,
        }
    }
}

use std::fmt::{Display, Formatter, Result};

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Pair({}, {})", self.0, self.1)
    }
}

use std::cmp::Ordering;

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        // We order pairs by their "best" member i.e. Pair::1
        self.1.cmp(&other.1)
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
