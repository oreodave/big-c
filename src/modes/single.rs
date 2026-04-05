use crate::card::Card;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Single(Card);

impl Single {
    /** Create a new single from a card `c`.  Will return None if a Single
    cannot be constructed from that card.

    The only situation where a card cannot be converted into a Single is if it's
    a Joker.
    */
    pub fn new(c: Card) -> Option<Single> {
        (!c.is_joker()).then_some(Single(c))
    }
}

use crate::modes::{Footstool, Hand};

impl Hand for Single {
    fn is_proper(&self) -> bool {
        // Always true as Jokers are not allowed
        true
    }

    fn footstool(&self, other: &Self) -> Footstool {
        // We use deck_abs() to get an index in the overall deck ordering.
        match (self.0.deck_abs(), other.0.deck_abs()) {
            // A full footstool only occurs when both are the same.
            (x, y) if x == y => Footstool::Full,
            // Half footstools can only occur when x is consecutive to y.
            (x, y) if x == y + 1 => Footstool::Half,
            _ => Footstool::None,
        }
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for Single {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Single[{}]", self.0)
    }
}

use std::hash::{Hash, Hasher};

impl Hash for Single {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;
    use crate::{
        card::{PlayingCard, Rank, Suit},
        modes::tests::test_footstool,
    };

    #[test]
    fn new() {
        // TEST: Jokers are not valid singles.
        assert_eq!(
            Single::new(Card::make_joker()),
            None,
            "Expected Jokers to never be valid singles"
        );

        let valid_singles = Card::iter_all(1)
            .filter_map(Single::new)
            .collect::<Vec<_>>();
        let deck = Card::iter_all(1).collect::<Vec<_>>();

        // TEST: Only two cards in a single deck aren't valid singles.
        assert_eq!(valid_singles.len(), deck.len() - 2);
    }

    #[test]
    fn footstool() {
        // Create a vector for all possible Singles in 1 deck of cards.  Due to
        // ordering of Card::iter_all, we expect this to be sorted as well.
        let singles = Card::iter_all(1)
            .filter_map(Single::new)
            .collect::<Vec<_>>();

        // TEST: Consecutive singles footstool testing.
        singles.windows(3).for_each(|single_slice| {
            let (s1, s2, s3) =
                (single_slice[0], single_slice[1], single_slice[2]);

            // TEST: Test footstool patterns and get some results back for
            // further testing.
            let (_, s2_on_s1) = test_footstool(&s1, &s2);
            let (_, s3_on_s2) = test_footstool(&s2, &s3);
            let (s1_on_s3, s3_on_s1) = test_footstool(&s1, &s3);

            // TEST: s2 is half-footstooled by s3, and s1 is half footstooled by
            // s2.
            assert!(
                s3_on_s2 == Footstool::Half,
                "{s3} should half footstool {s2}"
            );
            assert!(
                s2_on_s1 == Footstool::Half,
                "{s2} should half footstool {s1}"
            );

            // TEST: s1 does not footstool whatsoever with s3.
            assert!(
                s1_on_s3 == Footstool::None,
                "{s1} should not footstool {s3}"
            );
            assert!(
                s3_on_s1 == Footstool::None,
                "{s3} should not footstool {s1}"
            );
        });

        // Exhaustive testing over every possible combinations of Singles.

        // Create an exhaustive map for all combinations (Single, Single) along
        // with the results of the first footstooling the second.
        let exhaustive_singles_footstool = singles
            .iter()
            .flat_map(|single| {
                singles
                    .iter()
                    .map(move |other_single| (single, other_single))
            })
            .map(|(single, other_single)| {
                // TEST: Expected generic pattern for footstooling of hands -
                // see mod::tests::test_footstool for details.

                // NOTE: Due to test_footstool impl, this automatically tests
                // that single == other_single <=> single full footstools
                // other_single (and vice versa)
                (single, other_single, test_footstool(single, other_single).0)
            })
            .collect::<Vec<_>>();

        // TEST: Half footstools.
        {
            // Maps Singles to a Vector of the Singles they half footstool.
            let counter = {
                let mut counter: HashMap<Single, Vec<Single>> = HashMap::new();
                exhaustive_singles_footstool
                    .iter()
                    .filter(|(_, _, res)| *res == Footstool::Half)
                    .for_each(|(c1, c2, _)| {
                        if let Some(val) = counter.get_mut(*c1) {
                            val.push(**c2);
                        } else {
                            counter.insert(**c1, vec![**c2]);
                        }
                    });
                counter
            };

            // TEST: For any Single there is only 1 other Single that it
            // half footstools.
            counter.iter().for_each(|(c1, counter)| {
                assert_eq!(
                    counter.len(),
                    1,
                    "Expected {c1} to only have 1 card that it half footstools"
                );
            });

            // TEST: For any Single, the Single that it half footstools
            // is unique to it.
            {
                let mut unique_half_footstools = HashSet::<Single>::new();
                counter.iter().for_each(|(c1, counter)| {
                    let c2 = counter[0];
                    assert_eq!(unique_half_footstools.get(&c2), None, "Expected {c2} to be unique to the half footstools of {c1}");
                    unique_half_footstools.insert(c2);
                })
            }

            // TEST: The only Single that doesn't have a half footstool is 3[D]
            {
                let card = Single::new(Card::from(0)).unwrap();
                assert_eq!(
                    counter.get(&card),
                    None,
                    "Expected {card} to not have any half footstools."
                );
            }
        }

        // TEST: Non-footstools
        {
            // A little combinatorial check.  3[D] has no half footstools and 1
            // full footstool (itself).  Every other card should have 1 unique
            // half footstool and 1 full footstool.

            // 51 cards should satisfy the latter condition => 102 instances of
            // a half or full footstool.  With 3[D], that's 103 instances of a
            // footstool.

            // If the above conditions hold, then we'd expect the number of non
            // footstools in our exhaustive set to be (52 ** 2) - 103.

            assert_eq!(
                exhaustive_singles_footstool
                    .iter()
                    .filter(|(_, _, footstool)| *footstool == Footstool::None)
                    .count(),
                (52 * 52) - 103
            );
        }
    }

    #[test]
    fn footstool_deck_irrelevance() {
        // For a fixed Single, comparing to another deck's cards doesn't change
        // if it gets footstooled.
        let piv_card = Card::make_playing_card(Rank::Three, Suit::Club);
        let pivot = Single::new(piv_card).unwrap();

        for i in 1..10 {
            let piv_copy = Single(Card::PlayingCard(PlayingCard {
                deck: i,
                ..piv_card.playing_card().unwrap()
            }));

            let piv_before = Single(Card::from(i64::from(piv_copy.0) - 1));
            let piv_after = Single(Card::from(i64::from(piv_copy.0) + 1));
            let piv_way_after = Single(Card::from(i64::from(piv_copy.0) + 2));

            // TEST: a single may be footstooled by a single from another deck
            // with the same rank and suit.
            let (piv_on_piv_copy, _) = test_footstool(&pivot, &piv_copy);
            assert_eq!(
                piv_on_piv_copy,
                Footstool::Full,
                "Expected {pivot}, {piv_copy} to full footstool."
            );

            // TEST: A single may be half footstooled by singles from another
            // deck.
            let (piv_on_piv_before, _) = test_footstool(&pivot, &piv_before);
            assert_eq!(
                piv_on_piv_before,
                Footstool::Half,
                "Expected {pivot}, {piv_before} to half footstool."
            );

            let (_, piv_after_on_piv) = test_footstool(&pivot, &piv_after);
            assert_eq!(
                piv_after_on_piv,
                Footstool::Half,
                "Expected {pivot}, {piv_after} to half footstool."
            );

            // TEST: A single is still not footstooled by singles from other
            // decks that aren't adjacent.
            let (piv_on_piv_way_after, _) =
                test_footstool(&pivot, &piv_way_after);
            assert_eq!(
                piv_on_piv_way_after,
                Footstool::None,
                "Expected {pivot}, {piv_way_after} to not footstool."
            );
        }
    }
}
