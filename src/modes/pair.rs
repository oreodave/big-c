use crate::{card::Card, helper::ordered};

#[derive(Debug, Copy, Clone)]
pub struct Pair(Card, Card);

impl Pair {
    /// Create a new pair utilising two cards, `c1` and `c2`.  Will return None
    /// if a Pair cannot be constructed out of the two cards.
    ///
    /// NOTE: By construction, if the Pair includes a Joker, that Joker will be
    /// the first member of the pair.  In other words, Pair::1 will always be a
    /// valid playing card.
    pub fn new(c1: Card, c2: Card) -> Option<Pair> {
        let [c1, c2] = ordered([c1, c2]);

        match [c1, c2].map(|c| c.rank()) {
            // A Joker and a PlayingCard
            [None, Some(_)] => Some(Pair(c1, c2)),
            // Two PlayingCards of the same rank
            [Some(r1), Some(r2)] if r1 == r2 => Some(Pair(c1, c2)),
            _ => None,
        }
    }
}

use crate::helper::impl_cmp_eq_on_ord;
use std::cmp::Ordering;

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        /*
        Comparison is trivial: compare the top most card first.  If they're
        equivalent, then compare the last card to get a full ordering.
        Otherwise, use the high card comparison.
        */
        self.1.cmp(&other.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl_cmp_eq_on_ord!(Pair);

use crate::modes::{single::Single, Footstool, Hand};

impl Hand for Pair {
    fn is_proper(&self) -> bool {
        !self.0.is_joker()
    }

    fn high_card(&self) -> Card {
        self.1
    }

    fn footstool(&self, b: &Self) -> Footstool {
        match self.cmp(b) {
            // There is no footstool if self is beaten by other.
            Ordering::Less => Footstool::None,
            // We can only full footstool if we have equivalent pairs.
            Ordering::Equal => Footstool::Full,
            // Half footstools can proc if self.1 footstools other.1 (full or
            // half) using Single rules.
            Ordering::Greater => {
                // By construction, Pair::1 is always a playing card so we may
                // safely unwrap here.
                let [s1, s2] = [self, b]
                    .map(Hand::high_card)
                    .map(Single::new)
                    .map(Option::unwrap);
                match s1.footstool(&s2) {
                    Footstool::Full => Footstool::Half,
                    _ => Footstool::None,
                }
            }
        }
    }
}

use std::fmt::{Display, Formatter, Result};

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Pair[{}, {}]", self.0, self.1)
    }
}

use std::hash::{Hash, Hasher};

impl Hash for Pair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Pairs are just tuples lol.
        (self.0, self.1).hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        card::{PlayingCard, Rank},
        modes::tests::test_footstool,
        zipcartesian::ZipCartesianExt,
    };

    #[test]
    fn new() {
        /*
        There are 3 cases for generating a pair:
        1) Not a pair
        2) An improper pair made up of one joker
        3) A proper pair made up of two playing cards.
         */

        // TEST: Two jokers can never be a pair.
        assert_eq!(Pair::new(Card::make_joker(), Card::make_joker()), None);

        // TEST: Non pair tests.
        for (c1, c2) in Rank::iter_all()
            .into_iter()
            // Generate tuples (r1, r2) where r1 != r2
            .flat_map(|r1| {
                Rank::iter_all()
                    .into_iter()
                    .filter(move |&r2| r2 != r1)
                    .map(move |r2| (r1, r2))
            })
            // Generate all cards where their ranks differ
            .flat_map(|(r1, r2)| {
                r1.cards().into_iter().zip_cartesian(r2.cards().into_iter())
            })
        {
            // TEST: Two cards of differing rank cannot be a pair
            let pair = Pair::new(c1, c2);
            assert!(
                pair.is_none(),
                "Expected cards {c1} and {c2} to never form a pair."
            );
        }

        // TEST: Improper pair tests.
        for c1 in PlayingCard::iter_all(0).map(Card::PlayingCard) {
            // TEST: Any card with one joker can be made into a valid pair.
            let c2 = Card::make_joker();
            let pair = Pair::new(c1, c2);
            assert_ne!(pair, None, "Expected ({c1}, {c2}) to be a valid pair");

            let pair = pair.unwrap();

            // TEST: Pairs with a joker are improper.
            assert!(pair.is_improper(), "Expected {pair} to be improper");

            let Pair(c1, c2) = pair;

            // TEST: Improper pairs have a Joker in Pair::0 (c1).
            assert!(c1.is_joker(), "Expected {} to be a joker", pair.0);

            // TEST: Improper pairs have a playing card in Pair::1 (c2).
            assert!(!c2.is_joker(), "Expected {} to be a playing card", pair.1);
        }

        // TEST: Proper pair tests
        ;
        PlayingCard::iter_all(0)
            .map(Card::PlayingCard)
            .into_iter()
            // Flat Map every Playing Card (c1) into combinations (Card of same
            // rank as c1, c1)
            .flat_map(|c1| c1.rank().unwrap().cards().map(move |c2| (c1, c2)))
            .map(|(c1, c2)| {
                // TEST: Two cards of similar rank make a valid pair.
                let pair = {
                    let pair = Pair::new(c1, c2);
                    assert_ne!(
                        pair, None,
                        "Expected {c1} and {c2} to form a valid pair."
                    );
                    pair.unwrap()
                };
                (c1, c2, pair)
            })
            .for_each(|(c1, c2, pair)| {
                // TEST: Pairs of two playing cards are proper
                assert!(
                    pair.is_proper(),
                    "Expected {pair} from {c1} and {c2} to be proper."
                );

                let Pair(p1, p2) = pair;

                // TEST: Pairs always sort their cards in strength.
                let pair_cards = [p1, p2];
                let sorted_cards = ordered([c1, c2]);
                assert_eq!(
                    pair_cards, sorted_cards,
                    "Expected {} to be equivalent to ({}, {})",
                    pair, sorted_cards[0], sorted_cards[1]
                );
            });
    }

    /// Create an exhaustive set of pairs for one deck.
    fn exhaustive_pairs_deck() -> impl Iterator<Item = Pair> + Clone {
        Card::iter_all(1)
            .zip_cartesian(Card::iter_all(1))
            .filter_map(|(c1, c2)| Pair::new(c1, c2))
    }

    #[test]
    fn ordering() {
        fn expected_ordering_relation(p1: &Pair, p2: &Pair) -> bool {
            match (p1.cmp(p2), p1.1.cmp(&p2.1), p1.0.cmp(&p2.0)) {
                // For any two pairs, we expect the high cards to dictate the
                // ordering of the pairs first.  The low card only matters if
                // the high cards are equivalent.
                (x, Ordering::Equal, z) if x == z => true,
                (x, y, _) if x == y => true,
                _ => false,
            }
        }

        exhaustive_pairs_deck()
            .zip_cartesian(exhaustive_pairs_deck())
            .for_each(|(p1, p2)| {
                // TEST: For any two valid pairs we expect them to have the
                // `expected_ordering_relation`.
                assert!(expected_ordering_relation(&p1, &p2));
            })
    }

    #[test]
    fn footstool() {
        exhaustive_pairs_deck()
            .zip_cartesian(exhaustive_pairs_deck())
            .for_each(|(p1, p2)| {
                // TEST: Expected footstool condition
                test_footstool(&p1, &p2);
            });
    }
}
