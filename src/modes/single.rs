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
    fn is_proper(&self) -> bool {
        // Always true as Jokers are not allowed
        true
    }

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
    fn new() {
        // TEST: Jokers are not valid singles.
        assert!(Single::new(Card::joker()).is_none());

        let deck = make_decks(1);
        let singles: Vec<Option<Single>> =
            deck.iter().map(|&c| Single::new(c)).collect();
        let valid_singles: Vec<Single> = singles
            .iter()
            .filter(|x| !x.is_none())
            .map(|x| x.unwrap())
            .collect();

        // TEST: Only two cards in a single deck aren't valid singles.
        assert!(valid_singles.len() == deck.len() - 2);

        // TEST: All valid singles are playing cards.
        assert!(valid_singles.iter().all(|Single(card)| !card.is_joker()));
    }

    #[test]
    fn footstool() {
        let deck = make_decks(1);
        let deck = &deck[2..]; // skip the jokers
        let singles: Vec<Single> =
            deck.iter().map(|&c| Single::new(c).unwrap()).collect();

        singles.windows(3).for_each(|single_slice| {
            let (s1, s2, s3) =
                (single_slice[0], single_slice[1], single_slice[2]);

            // TEST: A single is always full footstooled by itself.
            assert!(s1.footstool(&s1) == Footstool::Full);

            // TEST: non-reflexivity of footstool on neighbours.
            let (_, s2_on_s1) = test_non_reflexivity(&s1, &s2);
            let (_, s3_on_s2) = test_non_reflexivity(&s2, &s3);
            let (s1_on_s3, s3_on_s1) = test_non_reflexivity(&s1, &s3);

            // TEST: s2 is half-footstooled by s3, and s1 is half footstooled by
            // s2.
            assert!(s3_on_s2 == Footstool::Half);
            assert!(s2_on_s1 == Footstool::Half);

            // TEST: s1 does not footstool whatsoever with s3
            assert!(s1_on_s3 == Footstool::None);
            assert!(s3_on_s1 == Footstool::None);
        });

        for single in &singles {
            let footstool_results: Vec<(Footstool, Footstool)> = singles
                .iter()
                .map(|&other_single| {
                    // TEST: All footstool results are non-reflexive.
                    test_non_reflexivity(single, &other_single)
                })
                .collect();

            let footstool_results: Vec<Footstool> =
                footstool_results.iter().map(|x| x.0).collect();

            // TEST: A single is only full-footstooled by itself.
            let full_footstools = footstool_results
                .iter()
                .filter(|&&x| x == Footstool::Full)
                .count();
            assert!(full_footstools == 1);

            // TEST: A single is half-footstooled by at most one singles
            let half_footstools = footstool_results
                .iter()
                .filter(|&&x| x == Footstool::Half)
                .count();
            assert!(half_footstools <= 1);

            // TEST: A single is not footstooled by any other singles.
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
    fn footstool_deck_irrelevance() {
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

            // TEST: a card may be footstooled by a card from another deck with
            // the same rank and suit.
            let (piv_on_piv_copy, _) = test_non_reflexivity(&pivot, &piv_copy);
            assert!(piv_on_piv_copy == Footstool::Full);

            // TEST: A card may be half footstooled by cards from another deck.
            let (piv_on_piv_before, _) =
                test_non_reflexivity(&pivot, &piv_before);
            assert!(piv_on_piv_before == Footstool::Half);

            let (_, piv_after_on_piv) =
                test_non_reflexivity(&pivot, &piv_after);
            assert!(piv_after_on_piv == Footstool::Half);

            // TEST: A card is still not footstooled by
            let (piv_on_piv_way_after, _) =
                test_non_reflexivity(&pivot, &piv_way_after);
            assert!(piv_on_piv_way_after == Footstool::None);
        }
    }
}
