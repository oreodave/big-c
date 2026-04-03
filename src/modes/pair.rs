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
    use crate::{
        card::{make_decks, Rank},
        modes::tests::test_non_reflexivity,
    };

    #[test]
    fn new() {
        // Two jokers can never be a pair.
        assert_eq!(Pair::new(Card::make_joker(), Card::make_joker()), None);

        // Iterate over all ranks
        for r1 in Rank::iter_all() {
            // Iterate over all cards in a rank
            for c1 in r1.cards() {
                // Iterate over all ranks that aren't r1
                for r2 in Rank::iter_all().filter(|&r2| r2 != r1) {
                    // TEST: Cannot compose a pair made from two different
                    // ranked cards.
                    let all_cards = r2.cards();
                    let all_pairs =
                        all_cards.filter_map(|c2| Pair::new(c1, c2));

                    assert_eq!(all_pairs.count(), 0);
                }

                // Iterate over all cards that are of the same rank as r1.
                for c2 in r1.cards() {
                    // TEST: Pairs are composed of two similar rank cards.
                    let pair = {
                        let pair = Pair::new(c1, c2);
                        assert_ne!(pair, None);
                        pair.unwrap()
                    };

                    // Test: Pairs of two playing cards are proper
                    assert!(pair.is_proper());

                    // TEST: Pairs always sort their cards in strength.
                    let (b1, b2) = ordered(c1, c2);
                    assert_eq!(pair.0, b1);
                    assert_eq!(pair.1, b2);
                }

                // TEST: Pairs may have one joker.
                let pair = {
                    let p = Pair::new(c1, Card::make_joker());
                    assert_ne!(p, None);
                    p.unwrap()
                };

                // TEST: Pairs with a joker are improper.
                assert!(pair.is_improper());

                // TEST: Improper pairs have a Joker in Pair::0.
                assert!(matches!(pair.0, Card::Joker(_)));
                assert!(matches!(pair.1, Card::PlayingCard(_)));
            }
        }
    }

    fn exhaustive_pairs_deck() -> impl Iterator<Item = Pair> {
        // A rank has 4 cards.  There are a total of 10 proper pairs that can be
        // made out of 4 cards.  With n jokers, add 4n improper pairs to the
        // total count of pairs for a single rank.
        make_decks(1).flat_map(|c1| {
            make_decks(1).filter_map(move |c2| Pair::new(c1, c2))
        })
    }

    #[test]
    fn ordering() {
        fn expected_ordering_relation(p1: &Pair, p2: &Pair) -> bool {
            let pair_ordering = p1.cmp(p2);
            let high_card_ordering = p1.1.cmp(&p2.1);

            match (pair_ordering, high_card_ordering) {
                // For any two pairs, we expect the high cards to dictate the
                // ordering of the pairs - the lower card should be irrelevant.
                (x, y) if x == y => true,

                // The only instances where the high card may not dictate card
                // ordering is if both pairs have equivalent high cards and
                // exclusively one of the pairs is improper - pairs that are
                // improper should be sorted less than the proper one.
                (Ordering::Less, Ordering::Equal) => {
                    // p1 has a joker, p2 doesn't => p1 < p2
                    p1.is_improper() && p2.is_proper()
                }
                (Ordering::Greater, Ordering::Equal) => {
                    // p2 has a joker, p1 doesn't => p1 > p2
                    p2.is_improper() && p1.is_proper()
                }
                _ => false,
            }
        }

        for p1 in exhaustive_pairs_deck() {
            for p2 in exhaustive_pairs_deck() {
                // TEST: For any two pair we expect them to have the
                // `expected_ordering_relation`.
                assert!(expected_ordering_relation(&p1, &p2));
            }
        }
    }

    #[test]
    fn footstool() {
        for p1 in exhaustive_pairs_deck() {
            for p2 in exhaustive_pairs_deck() {
                let (p1_on_p2, p2_on_p1) = test_non_reflexivity(&p1, &p2);
                let (hc1_on_hc2, hc2_on_hc1) = {
                    let [high_card_1, high_card_2] =
                        [p1.1, p2.1].map(Single::new).map(|x| {
                            assert!(x.is_some());
                            x.unwrap()
                        });
                    test_non_reflexivity(&high_card_1, &high_card_2)
                };

                // TEST: We expect pair footstools to mirror footstool rules of
                // Singles on the high card.
                assert!(p1_on_p2 == hc1_on_hc2 && p2_on_p1 == hc2_on_hc1);
            }
        }
    }
}
