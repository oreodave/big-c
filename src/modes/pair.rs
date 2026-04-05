use crate::{
    card::{Card, PlayingCard},
    helper::ordered,
};

#[derive(Eq, Debug, Copy, Clone)]
pub struct Pair(Card, Card);

impl Pair {
    /** Create a new pair utilising two cards, `c1` and `c2`.  Will return None
    if a Pair cannot be constructed out of the two cards.

    NOTE: By construction, if the Pair includes a Joker, that Joker will be the
    first member of the pair.  In other words, Pair::1 will always be a valid
    playing card.
     */
    pub fn new(c1: Card, c2: Card) -> Option<Pair> {
        // Order the cards.  This means if xor(c1 is joker, c2 is joker) c1 will
        // be that joker.
        let [c1, c2] = ordered([c1, c2]);

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

use crate::modes::{single::Single, Footstool, Hand};

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
        Comparison is trivial: compare the top most card first.  If they're
        equivalent, then compare the last card to get a full ordering.
        Otherwise, use the high card comparison.
        */
        let high_card_comp = self.1.cmp(&other.1);
        if high_card_comp == Ordering::Equal {
            self.0.cmp(&other.0)
        } else {
            high_card_comp
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
    use crate::{card::Rank, modes::tests::test_footstool};

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
        Rank::iter_all()
            // Generate a mapping (r1, r2) where r1, r2 are ranks and r1 != r2
            .flat_map(|r1| {
                Rank::iter_all()
                    .filter(move |&r2| r2 != r1)
                    .map(move |r2| (r1, r2))
            })
            // Flat Map for combinations of (cards of rank r1, cards of rank r2)
            .flat_map(|(r1, r2)| {
                r1.cards()
                    .flat_map(move |c1| r2.cards().map(move |c2| (c1, c2)))
            })
            .for_each(|(c1, c2)| {
                // TEST: Two cards of differing rank cannot be a pair
                let pair = Pair::new(c1, c2);
                assert!(
                    pair.is_none(),
                    "Expected cards {c1} and {c2} to never form a pair."
                )
            });

        // TEST: Improper pair tests.
        PlayingCard::iter_all(1)
            .map(Card::PlayingCard)
            .map(|c1| {
                // TEST: Any card with one joker can be made into a valid pair.
                let c2 = Card::make_joker();
                let pair = Pair::new(c1, c2);
                assert!(
                    pair.is_some(),
                    "Expected ({c1}, {c2}) to be a valid pair",
                );
                pair.unwrap()
            })
            .for_each(|pair| {
                // TEST: Pairs with a joker are improper.
                assert!(pair.is_improper(), "Expected {pair} to be improper");

                // TEST: Improper pairs have a Joker in Pair::0.
                assert!(
                    matches!(pair.0, Card::Joker(_)),
                    "Expected {} to be a joker",
                    pair.0
                );

                // TEST: Improper pairs have a playing card in Pair::1.
                assert!(
                    matches!(pair.1, Card::PlayingCard(_)),
                    "Expected {} to be a playing card",
                    pair.1
                );
            });

        // TEST: Proper pair tests
        PlayingCard::iter_all(1)
            .map(Card::PlayingCard)
            // Flat Map every card (c1) into combinations (c1, card of same rank as c1)
            .flat_map(|c1| c1.rank().unwrap().cards().map(move |c2| (c1, c2)))
            // Map every (c1, c2) into a pair
            .map(|(c1, c2)| {
                // TEST: Two cards of similar rank make a valid pair.
                let pair = {
                    let pair = Pair::new(c1, c2);
                    assert!(
                        pair.is_some(),
                        "Expected {c1} and {c2} to form a valid pair."
                    );
                    pair.unwrap()
                };
                // TEST: Pairs of two playing cards are proper
                assert!(pair.is_proper(), "Expected {pair} to be proper.");
                (c1, c2, pair)
            })
            .for_each(|(c1, c2, pair)| {
                // TEST: Pairs always sort their cards in strength.
                let [b1, b2] = ordered([c1, c2]);
                assert_eq!(pair.0, b1, "Expected {} to be {b1}", pair.0);
                assert_eq!(pair.1, b2, "Expected {} to be {b2}", pair.1);
            });
    }

    /** Create an exhaustive set of pairs for one deck. */
    fn exhaustive_pairs_deck() -> impl Iterator<Item = Pair> {
        Card::iter_all(1).flat_map(|c1| {
            Card::iter_all(1).filter_map(move |c2| Pair::new(c1, c2))
        })
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
            // Create a flat map of all combinations of two valid pairs.
            .flat_map(|p1| exhaustive_pairs_deck().map(move |p2| (p1, p2)))
            .for_each(|(p1, p2)| {
                // TEST: For any two valid pairs we expect them to have the
                // `expected_ordering_relation`.
                assert!(expected_ordering_relation(&p1, &p2));
            })
    }

    #[test]
    fn footstool() {
        exhaustive_pairs_deck()
            .flat_map(|p1| exhaustive_pairs_deck().map(move |p2| (p1, p2)))
            .for_each(|(p1, p2)| {
                // TEST: Expected footstool condition
                test_footstool(&p1, &p2);
            });
    }
}
