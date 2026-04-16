use crate::{card::Card, helper::ordered};

#[derive(Debug, Copy, Clone)]
pub struct Triple(Card, Card, Card);

impl Triple {
    /// Create a new triple utilising 3 cards: `c1`, `c2`, and `c3`.  Will
    /// return None iff a Triple cannot be constructed out of those 3 cards.
    ///
    /// NOTE: By construction, if a triple includes 1 Joker, then Triple::0 is
    /// that joker.  If a triple includes 2 jokers, then Triple::0 and Triple::1
    /// are those jokers.  This means Triple::2 will always be a valid playing
    /// card.
    pub fn new(c1: Card, c2: Card, c3: Card) -> Option<Triple> {
        let [c1, c2, c3] = ordered([c1, c2, c3]);

        match [c1, c2, c3].map(|c| c.rank()) {
            [None, None, Some(_)] => Some(Triple(c1, c2, c3)),
            [None, Some(r2), Some(r3)] if r2 == r3 => Some(Triple(c1, c2, c3)),
            [Some(r1), Some(r2), Some(r3)] if r1 == r2 && r2 == r3 => {
                Some(Triple(c1, c2, c3))
            }
            _ => None,
        }
    }

    pub fn high_pair(&self) -> Pair {
        Pair::new(self.1, self.2).unwrap()
    }

    fn count_jokers(&self) -> usize {
        [self.0, self.1, self.2]
            .iter()
            .filter(|c| c.is_joker())
            .count()
    }
}

use crate::helper::impl_cmp_eq_on_ord;
use std::cmp::Ordering;

impl Ord for Triple {
    fn cmp(&self, other: &Self) -> Ordering {
        let Triple(s1, s2, s3) = self;
        let Triple(o1, o2, o3) = other;

        // The most critical part of ordering is top card rank comparison.
        s3.rank()
            .cmp(&o3.rank())
            // if we have 2 triples, both with the same ranked high card, and
            // one has 2 jokers while the other doesn't => the 2 joker triple is
            // worse.
            .then_with(|| match (self.count_jokers(), other.count_jokers()) {
                (2, x) if x < 2 => Ordering::Less,
                (x, 2) if x < 2 => Ordering::Greater,
                _ => Ordering::Equal,
            })
            // then compare by the highest to lowest cards
            .then_with(|| s3.suit().cmp(&o3.suit()))
            .then_with(|| s2.cmp(o2))
            .then_with(|| s1.cmp(o1))
    }
}

impl_cmp_eq_on_ord!(Triple);

use crate::modes::{pair::Pair, Footstool, Hand};

impl Hand for Triple {
    fn is_proper(&self) -> bool {
        self.count_jokers() == 0
    }

    fn high_card(&self) -> Card {
        self.2
    }

    fn footstool(&self, other: &Self) -> Footstool {
        match self.cmp(other) {
            // There is no footstool if self is beaten by other.
            Ordering::Less => Footstool::None,
            // We can only full footstool if we have equivalent triples.
            Ordering::Equal => Footstool::Full,
            // Half footstools can only proc if the 2 high cards of each hand
            // footstool each other using Pair rules.
            Ordering::Greater => {
                // By construction, Triple::1 and Triple::2 should always make a
                // Pair so it's cool to unwrap.
                let [p1, p2] = [self, other].map(|x| x.high_pair());
                match p1.footstool(&p2) {
                    Footstool::Full => Footstool::Half,
                    _ => Footstool::None,
                }
            }
        }
    }
}

use std::fmt::{Display, Formatter, Result};

impl Display for Triple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Triple[{}, {}, {}]", self.0, self.1, self.2)
    }
}

use std::hash::{Hash, Hasher};

impl Hash for Triple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Pairs are just tuples lol.
        (self.0, self.1, self.2).hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        card::{PlayingCard, Rank},
        modes::tests::test_footstool,
        zipcartesian::ZipCartesianExt,
    };

    use super::*;

    #[test]
    fn new() {
        let joker = Card::make_joker();

        // TEST: Cannot make a triple out of three jokers
        assert_eq!(
            Triple::new(joker, joker, joker),
            None,
            "Expected triple of 3 jokers to be None"
        );

        for card in PlayingCard::iter_all(0).map(Card::PlayingCard) {
            let trip = Triple::new(card, joker, joker);
            // TEST: Any card with two jokers is a triple
            assert_ne!(
                trip, None,
                "Expected ({card}, {joker}, {joker}) to make a triple"
            );
            let trip = trip.unwrap();

            // TEST: Triples formed with 2 jokers are improper.
            assert!(trip.is_improper(), "Expected {trip} to be improper");

            // TEST: Triples with 2 jokers should have a playing card for
            // Triple::2.
            assert_eq!(
                trip.2, card,
                "Expected Triple::2 to be the sole PlayingCard"
            );

            assert_eq!(
                trip.high_card(),
                card,
                "Expected Triple::HighCard = card"
            );
        }

        // Iterate over all pairs of cards with similar ranks
        for (c1, c2) in Rank::all().into_iter().flat_map(|r| {
            r.cards().into_iter().zip_cartesian(r.cards().into_iter())
        }) {
            let trip = Triple::new(c1, c2, joker);
            // TEST: Any two similar rank cards with 1 joker is a Triple.
            assert_ne!(
                trip, None,
                "Expected ({c1}, {c2}, Joker) to make a Triple"
            );

            let trip = trip.unwrap();

            // TEST: Triples formed with 1 joker are improper.
            assert!(trip.is_improper(), "Expected {trip} to be improper");

            // TEST: 1 joker triples have Triple::0 as the joker.
            assert!(
                matches!(trip.0, Card::Joker(_)),
                "Expected {} to be a joker",
                trip.0
            );

            let [c1, c2] = ordered([c1, c2]);

            // TEST: Expect Triple::1 and Triple::2 to follow ordering
            // of c1 and c2.
            assert_eq!(
                [trip.1, trip.2],
                [c1, c2],
                "Expected {} = min({c1}, {c2}) and {} = max({c1}, {c2})",
                trip.1,
                trip.2,
            );

            // TEST: Expect triple high card to be the highest card of c1,c2.
            assert_eq!(trip.high_card(), c2);
        }

        // Iterate over all pairs of cards with differing ranks
        for (c1, c2) in Rank::all()
            .into_iter()
            .flat_map(|r1| {
                Rank::all()
                    .into_iter()
                    .filter(move |&r2| r2 != r1)
                    .map(move |r2| (r1, r2))
            })
            .flat_map(|(r1, r2)| {
                r1.cards().into_iter().zip_cartesian(r2.cards().into_iter())
            })
        {
            // TEST: Cannot make a triple out of 1 joker and two different rank
            // cards
            let trip = Triple::new(c1, c2, joker);
            assert_eq!(
                trip, None,
                "Expected ({c1}, {c2}, {joker}) to never make a triple."
            );
        }

        // Iterate over all triples of cards (regardless of rank)
        for (c1, (c2, c3)) in
            PlayingCard::iter_all(0).into_iter().zip_cartesian(
                PlayingCard::iter_all(0)
                    .into_iter()
                    .zip_cartesian(PlayingCard::iter_all(0).into_iter()),
            )
        {
            let [c1, c2, c3] = [c1, c2, c3].map(Card::PlayingCard);
            let trip = Triple::new(c1, c2, c3);

            // TEST: Any 3 playing cards make a triple iff they match in
            // rank
            if !(c1.rank() == c2.rank() && c2.rank() == c3.rank()) {
                assert_eq!(
                    trip, None,
                    "Expected {c1}, {c2}, {c3} to never make a Triple."
                );
                continue;
            } else {
                assert_ne!(
                    trip, None,
                    "Expected {c1}, {c2}, {c3} to make a Triple."
                );
            }

            let trip = trip.unwrap();
            // TEST: Triples formed of 3 playing cards are proper.
            assert!(trip.is_proper(), "Expected {trip} to be proper");

            let [c1, c2, c3] = ordered([c1, c2, c3]);

            // TEST: Triple::high_card should be c3 (the highest card of the
            // ordered set).
            assert_eq!(trip.high_card(), c3);

            // TEST: If a triple is formed of 3 playing cards, they are
            // ordered s.t. Triple::2 > Triple::1 > Triple::0.
            assert_eq!(
                [trip.0, trip.1, trip.2],
                [c1, c2, c3],
                "Expected cards of {} to match ordered cards [{}, {}, {}]",
                trip,
                c1,
                c2,
                c3
            );
        }
    }

    fn exhaustive_triples_deck() -> impl Iterator<Item = Triple> + Clone {
        Card::iter_all(1)
            .zip_cartesian(Card::iter_all(1).zip_cartesian(Card::iter_all(1)))
            .filter_map(|(c1, (c2, c3))| Triple::new(c1, c2, c3))
    }

    fn exhaustive_triples_rank(
        rank: Rank,
    ) -> impl Iterator<Item = Triple> + Clone {
        let choices = rank.cards().into_iter().chain([Card::make_joker()]);
        let c1s = choices.clone();
        let c2s = choices.clone();
        let c3s = choices.clone();
        c1s.zip_cartesian(c2s.zip_cartesian(c3s))
            .filter_map(|(c1, (c2, c3))| Triple::new(c1, c2, c3))
    }

    #[test]
    fn ordering() {
        // Unlike pairs and singles, the complexity of triples ordering is too
        // great to be exhaustive.  As a compromise, we'll run through a bunch
        // of cases that should demonstrate everything we want.

        let joker = Card::make_joker();

        for (t1, t2) in
            // Generate an exhaustive set of triples where rank(t1) <
            // rank(t2)
            Rank::all()
                .into_iter()
                .flat_map(move |r1| r1.iter_rest().map(move |r2| (r1, r2)))
                .flat_map(|(r1, r2)| {
                    exhaustive_triples_rank(r1)
                        .zip_cartesian(exhaustive_triples_rank(r2))
                })
        {
            // TEST: Any triple formed of some higher rank will be better than
            // any triple formed of a lower rank.
            assert!(t2 > t1, "Expected {t1} < {t2} as rank({t1}) < rank({t2})");
            assert!(t1 < t2, "Expected {t2} > {t1} as rank({t1}) < rank({t2})");
        }

        // So high card rank determines ordering between differing ranked
        // triples.  Let's test what happens within triples of the same high
        // card rank.

        // Iterate through all ranks
        for rank in Rank::all() {
            let cards = rank.cards();

            let [diamond, _, _, spade] = cards;

            // NOTE: By new test this should be safe to unwrap.
            let minima = Triple::new(diamond, joker, joker).unwrap();
            let maxima = Triple::new(spade, spade, spade).unwrap();

            // All possible 2 joker triples for this rank.
            let two_joker_triples = cards
                .map(|c| Triple::new(c, joker, joker))
                .map(Option::unwrap);

            for triple in exhaustive_triples_rank(rank) {
                // TEST: The lowest possible triple in a rank is a diamond + 2
                // jokers
                assert!(minima <= triple);
                // TEST: The highest possible triple in a rank is 3 spades
                assert!(maxima >= triple);

                if triple.count_jokers() < 2 {
                    for two_joker_trip in two_joker_triples {
                        // TEST: A two joker triple is always worse than any
                        // triples in the same rank that have at most 1 joker.
                        assert!(two_joker_trip < triple);
                    }
                }
            }
        }
    }

    #[test]
    fn footstool() {
        let triples = exhaustive_triples_deck().collect::<Vec<_>>();
        for (ind, t1) in triples.iter().enumerate() {
            for t2 in &triples[ind..] {
                // TEST: Expected footstool condition
                test_footstool(t1, t2);
            }
        }
    }
}
