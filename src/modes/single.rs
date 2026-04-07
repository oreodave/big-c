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
        matches!(c, Card::PlayingCard(_)).then_some(Single(c))
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
    use crate::modes::tests::test_footstool;

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
        /*
        A combinatorial check should be satisfactory here, given the extensive
        testing of in-deck footstools from the previous test.  I'll create a
        formula for the number of footstools we should expect to see from an
        exhaustive checking of all Singles in N decks of cards where N > 0.

        For 1 deck there are 51 cards that have 1 half footstool and 1 full
        footstool, and 1 card (3[D]) which has 1 full footstool.  This gave us
        103 by (51 * 2) + 1.

        For n decks, there will be (51*n) cards that have n choices for full
        footstool and n choices for half footstool.  The remaing n cards will
        all be 3[D], which means they will only get n full footstools.

        Another way to look at it is (52 * n * n) half footstools and (51 * n *
        n) full footstools.  So (51 + 52) * (n * n) => 103n^2 footstools in n
        decks.
         */

        const N_DECKS: usize = 10;

        // Function which maps a Footstool to a usize for use in array indexing.
        fn footstool_to_numeral(f: Footstool) -> usize {
            match f {
                Footstool::None => 0,
                Footstool::Half => 1,
                Footstool::Full => 2,
            }
        }

        // Create a map that counts all the footstools possible between all
        // cards.
        let counter = {
            let mut counter = [0; 3];
            // Iterator over Cards from N_DECKS number of decks.
            let cards = (0..((N_DECKS as i64) * 52)).map(Card::from);
            cards
                .clone()
                .flat_map(move |c1| cards.clone().map(move |c2| (c1, c2)))
                .for_each(|(c1, c2)| {
                    let res = test_footstool(&Single(c1), &Single(c2)).0;
                    let res = footstool_to_numeral(res);
                    counter[res] += 1;
                });

            counter
        };

        let [_, half, full] = counter;

        // TEST: There are 103n^2 footstool instances.
        assert_eq!(103 * N_DECKS * N_DECKS, half + full);

        // TEST: There are 51n^2 half footstool instances.
        assert_eq!(51 * N_DECKS * N_DECKS, half);

        // TEST: There are 52n^2 full footstool instances.
        assert_eq!(52 * N_DECKS * N_DECKS, full);
    }
}
