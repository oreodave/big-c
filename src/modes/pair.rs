use crate::card::{Card, PlayingCard};
use crate::helper::ordered;

#[derive(Eq, Debug, Copy, Clone)]
pub struct Pair(pub Card, pub Card);

impl Pair {
    /** Create a new pair utilising two cards, `c1` and `c2`.  Will return None
    if a Pair cannot be constructed out of the two cards.

    NOTE: By construction, if the Pair includes a Joker, that Joker will be the
    first member of the pair.  In other words, Pair::1 will always be a valid
    playing card.
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

use crate::modes::single::Single;
use crate::modes::{Footstool, Hand};

impl Hand for Pair {
    fn is_proper(&self) -> bool {
        matches!(self.0, Card::PlayingCard(_))
    }

    fn footstool(&self, other: &Self) -> Footstool {
        // A pair footstools the other <=> the highest cards of both footstool
        // each other => we can rely on the footstool implementation of Single
        // for this.
        Single(self.1).footstool(&Single(other.1))
    }
}

use std::fmt::{Display, Formatter, Result};

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Pair[{}, {}]", self.0, self.1)
    }
}

use std::cmp::Ordering;

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        /*
        Comparison is slightly complicated by the inclusion of wild cards.
        Rules are as follows:
        1) Two proper/improper pairs that have equivalent high cards are
           equivalent.
        2) Two pairs with equivalent high cards but only one is proper; the pair
           that is proper wins.
        3) Otherwise, comparison between high cards is the ordering.
        */
        match (self.1.cmp(&other.1), self.is_proper(), other.is_proper()) {
            (Ordering::Equal, false, true) => Ordering::Less,
            (Ordering::Equal, true, false) => Ordering::Greater,
            (Ordering::Equal, ..) => Ordering::Equal,
            (x, ..) => x,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::make_decks;

    fn exhaustive_pairs() -> Vec<Pair> {
        let deck = make_decks(1);
        // Each rank (4 cards) can form:
        // 1) 10 proper pairs: 4 self pairs + 6 pairs with different suits
        // 2) 8 improper pairs: 2 jokers * 4 cards in a rank.
        // => 18 pairs per rank.
        let mut pairs: Vec<Pair> = Vec::with_capacity(13 * 18);

        // This is technically a bunch of wasted effort, but for the sake of
        // testing it's necessary.
        for i in 0..deck.len() {
            let c1 = deck[i];
            for j in i..deck.len() {
                let c2 = deck[j];
                let pair = Pair::new(c1, c2);
                if pair.is_some() {
                    pairs.push(pair.unwrap());
                }
            }
        }
        pairs
    }

    #[test]
    fn new() {
        todo!("Implement tests for Pair::new");
    }

    #[test]
    fn ordering() {
        todo!("Implement tests for Pair ordering");
    }

    #[test]
    fn footstool() {
        todo!("Implement tests for Pair footstools");
    }

    #[test]
    fn footstool_deck_irrelevance() {
        todo!("Implement tests for Pair footstool deck irrelevance");
    }
}
