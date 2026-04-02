use crate::card::Card;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Single(pub Card);

impl Single {
    /** Create a new single from a card `c`.  Will return None if a Single
    cannot be constructed from that card.

    The only situation where a card cannot be converted into a Single is if it's
    a Joker.
    */
    fn new(c: Card) -> Option<Single> {
        (!c.is_joker()).then_some(Single(c))
    }
}

use crate::modes::{Footstool, Hand};

impl Hand for Single {
    fn footstool(&self, other: &Self) -> Footstool {
        let self_abs = self.0.deck_abs();
        let other_abs = other.0.deck_abs();

        if self_abs == other_abs {
            Footstool::Full
        } else if self_abs == other_abs + 1 {
            Footstool::Half
        } else {
            Footstool::None
        }
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for Single {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Single[{}]", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        card::{make_decks, PlayingCard, Rank, Suit},
        modes::tests::test_non_reflexivity,
    };

    #[test]
    fn invalid_singles() {
        let deck = make_decks(1);
        let singles: Vec<Option<Single>> =
            deck.iter().map(|&c| Single::new(c)).collect();
        let valid_singles: Vec<Single> = singles
            .iter()
            .filter(|x| !x.is_none())
            .map(|x| x.unwrap())
            .collect();

        // There are exactly two cards in a single deck that aren't valid
        // singles.  In other words, all other cards are valid.
        assert!(valid_singles.len() == deck.len() - 2);

        // All valid singles are playing cards.
        assert!(valid_singles.iter().all(|Single(card)| !card.is_joker()));

        // By the previous two results, the only invalid singles are jokers.  A
        // direct test of this is fine as well.
        assert!(Single::new(Card::from(-1)).is_none());
    }

    #[test]
    fn footstools() {
        let deck = make_decks(1);
        let deck = &deck[2..]; // skip the jokers
        let singles: Vec<Single> =
            deck.iter().map(|&c| Single::new(c).unwrap()).collect();

        singles.windows(3).for_each(|single_slice| {
            let (s1, s2, s3) =
                (single_slice[0], single_slice[1], single_slice[2]);

            // A single is always full footstooled by itself
            assert!(s1.footstool(&s1) == Footstool::Full);

            // Test general non-reflexivity of the footstool relation and get
            // back some results we'd like to verify further.
            let (_, s2_on_s1) = test_non_reflexivity(&s1, &s2);
            let (_, s3_on_s2) = test_non_reflexivity(&s2, &s3);
            let (s1_on_s3, s3_on_s1) = test_non_reflexivity(&s1, &s3);

            // s2 is half-footstooled by s3, and s1 is half footstooled by s2.
            assert!(s3_on_s2 == Footstool::Half);
            assert!(s2_on_s1 == Footstool::Half);
            // s1 does not footstool whatsoever with s3
            assert!(s1_on_s3 == Footstool::None);
            assert!(s3_on_s1 == Footstool::None);
        });

        // An exhaustive check to verify that:
        // 1) All footstool results are not reflexive.
        // 2) A single is ONLY full-footstooled by itself
        // 3) A single is half-footstooled by at most one singles
        // 4) A single is not footstooled by any other singles
        for single in &singles {
            // Check footstools against every other card
            let footstool_results: Vec<(Footstool, Footstool)> = singles
                .iter()
                .map(|&other_single| {
                    // We verify (1) here
                    test_non_reflexivity(single, &other_single)
                })
                .collect();

            let footstool_results: Vec<Footstool> =
                footstool_results.iter().map(|x| x.0).collect();

            // (2)
            let full_footstools = footstool_results
                .iter()
                .filter(|&&x| x == Footstool::Full)
                .count();
            assert!(full_footstools == 1);

            // (3)
            let half_footstools = footstool_results
                .iter()
                .filter(|&&x| x == Footstool::Half)
                .count();
            assert!(half_footstools <= 1);

            // (4)
            let non_footstools = footstool_results
                .iter()
                .filter(|&&x| x == Footstool::None)
                .count();
            assert!(
                non_footstools < singles.len()
                    && non_footstools >= singles.len() - 3
            );
        }
    }

    #[test]
    fn deck_irrelevance() {
        // For a fixed card, comparing to another deck's cards doesn't change if
        // it gets footstooled.
        let pivot = PlayingCard::new(0, Rank::Three, Suit::Club);
        let pivot = Card::PlayingCard(pivot);
        let pivot = Single(pivot);
        for i in 1..10 {
            let piv_copy = Single(Card::PlayingCard(PlayingCard {
                deck: i,
                ..pivot.0.playing_card().unwrap()
            }));
            let piv_before = Single(Card::from(i64::from(piv_copy.0) - 1));
            let piv_after = Single(Card::from(i64::from(piv_copy.0) + 1));
            let piv_way_after = Single(Card::from(i64::from(piv_copy.0) + 2));

            assert!(pivot.footstool(&piv_copy) == Footstool::Full);
            assert!(pivot.footstool(&piv_before) == Footstool::Half);
            assert!(piv_after.footstool(&pivot) == Footstool::Half);
            assert!(pivot.footstool(&piv_way_after) == Footstool::None);
        }
    }
}
