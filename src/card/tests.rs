#[cfg(test)]
mod test_numerics {
    use std::collections::HashSet;

    use crate::card::{Card, PlayingCard, Rank, Suit};

    #[test]
    fn rank() {
        // TEST: Negative numbers cannot be ranks
        assert!(matches!(Rank::try_from(-1), Err(_)));

        // TEST: Numbers >= 13 cannot be a rank.
        for i in 13..1000 {
            assert!(
                matches!(Rank::try_from(i), Err(_)),
                "Expected {i} not to match a rank"
            );
        }

        // TEST: Numbers in [0, 12] are mapped to ranks
        let set = (0..13)
            .map(|i| {
                let rank = Rank::try_from(i);
                assert!(rank.is_ok(), "Expected {i} to map to a valid rank");
                rank.unwrap()
            })
            .collect::<HashSet<_>>();

        assert_eq!(
            set.len(),
            13,
            "Expected 13 unique ranks from Rank::try_from(0..13)"
        );
    }

    #[test]
    fn suit() {
        // TEST: Negative numbers cannot be suits
        assert!(matches!(Suit::try_from(-1), Err(_)));

        // TEST: Numbers >= 4 cannot be a suit.
        for i in 4..1000 {
            assert!(
                matches!(Suit::try_from(i), Err(_)),
                "Expected {i} not to match a suit"
            );
        }

        // TEST: Numbers in [0, 3] are mapped to suits
        let set = (0..4)
            .map(|i| {
                let suit = Suit::try_from(i);
                assert!(suit.is_ok(), "Expected {i} to map to a valid suit");
                suit.unwrap()
            })
            .collect::<HashSet<_>>();

        assert_eq!(
            set.len(),
            4,
            "Expected 4 unique Suits from Suit::try_from(0..4)"
        );
    }

    #[test]
    fn playing_card() {
        // TEST: Negative numbers cannot be playing cards
        assert!(PlayingCard::try_from(-1).is_err());

        // TEST: PlayingCard derived from 0 should be the 3 of Diamonds of the
        // 0th deck.
        {
            let expected = PlayingCard::new(0, Rank::Three, Suit::Diamond);
            let card = PlayingCard::try_from(0);
            assert!(card.is_ok());
            let card = card.unwrap();
            assert_eq!(card.rank, expected.rank);
            assert_eq!(card.suit, expected.suit);
        }

        fn range_to_playing_card(deck: i64) -> HashSet<PlayingCard> {
            ((deck * 52)..((deck + 1) * 52))
                .map(|n| {
                    let res = PlayingCard::try_from(n);
                    assert!(
                        res.is_ok(),
                        "Expected {n} to map to a PlayingCard"
                    );
                    res.unwrap()
                })
                .map(|card| {
                    // TEST: Playing cards derived from 52d..52(d + 1) all are
                    // in deck d.
                    assert_eq!(card.deck, deck);
                    card
                })
                .collect::<HashSet<_>>()
        }

        // 0..51 should map to the 0th deck of playing cards.
        let first_deck = range_to_playing_card(0);
        // 52..103 should map to the 1st deck of playing cards.
        let second_deck = range_to_playing_card(1);

        // TEST: Expect 52 unique playing cards from
        // PlayingCard::try_from(0..52) or PlayingCard::try_from(52..104).
        assert_eq!(first_deck.len(), 52, "Expected 52 unique cards");
        assert_eq!(second_deck.len(), 52, "Expected 52 unique cards");

        // There are 52 unique cards in PlayingCard::try_from(0..52).  With deck
        // being fixed to one number, pigeonhole principle suggests all Playing
        // Cards really must be there (13 ranks, 4 suits => 52 combinations).
        // To really prove it we can check all combinations of rank and suit and
        // observe that they are present.

        for rank in (0..13i64).map(|n| Rank::try_from(n).unwrap()) {
            for suit in (0..4i64).map(|n| Suit::try_from(n).unwrap()) {
                let playing_card = PlayingCard::new(0, rank, suit);
                // TEST: Expected combination of rank and suit to be present in
                // first deck.
                assert!(
                    first_deck.contains(&playing_card),
                    "Expected {playing_card} to be present in the map of 0th deck"
                );

                // TEST: Expected combination of rank and suit to be present in
                // second deck.
                let playing_card = PlayingCard::new(1, rank, suit);
                assert!(
                    second_deck.contains(&playing_card),
                    "Expected {playing_card} to be present in the map of 1st deck"
                );
            }
        }

        // To test i64::from on PlayingCard, we can just do a map over a big
        // range and see that PlayingCard::try_from is the inverse.  This range
        // should cover an inordinate number of Cards for a single game so for
        // our purposes it's just fine.
        for i in 0..1000 {
            let pc = PlayingCard::try_from(i).unwrap();
            let ret = i64::from(pc);
            // TEST: i64::from is the exact inverse of PlayingCard::try_from.
            assert_eq!(
                i, ret,
                "Expected i64::from(PlayingCard::try_from(x)) = x"
            );
        }
    }

    #[test]
    fn card() {
        // You can make Cards from negative numbers.
        for i in -100..0 {
            let card = Card::from(i);

            // TEST: Card::from(negative number) makes jokers.
            assert!(
                matches!(card, Card::Joker(_)),
                "Expected Card::from({i}) makes jokers"
            );
        }

        // Card::from should defer to PlayingCard::try_from for positive
        // integers.  We may test this with a ridiculous range.
        let range = 0..1000;

        let cards = range.clone().map(Card::from).collect::<Vec<_>>();

        let playing_cards = range
            .clone()
            .map(|x| PlayingCard::try_from(x).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(cards.len(), playing_cards.len());
        assert_eq!(cards.len(), 1000);

        for (card, pc) in cards.iter().zip(playing_cards.iter()) {
            if let Card::PlayingCard(c) = card {
                assert_eq!(c.deck, pc.deck);
                assert_eq!(c.rank, pc.rank);
                assert_eq!(c.suit, pc.suit);
            } else {
                unreachable!("All of cards should be playing cards");
            }
        }

        // Thus the test on PlayingCard::from applies here.  Great!

        // To test i64::from on Card, we'll do what the test on PlayingCard did
        // and use a massive range.
        for i in -1000..1000 {
            let pc = Card::from(i);
            let ret = i64::from(pc);
            // TEST: i64::from is the exact inverse of Card::from.
            assert_eq!(i, ret, "Expected i64::from(Card::from({i})) = {i}");
        }
    }
}

#[cfg(test)]
mod test_ord {
    use crate::card::Card;

    #[test]
    fn jokers() {
        // All jokers are equivalent, regardless of the i64 they originate from.
        let joker = Card::from(-1);
        for i in -1000..-1 {
            let other = Card::from(i);
            assert_eq!(joker, other);
        }

        // All playing cards are "better" than Jokers.
        for i in 0..1000 {
            let other = Card::from(i);
            assert!(other > joker, "{other} > {joker}");
            assert!(joker < other, "{joker} < {other}");
        }
    }

    #[test]
    fn natural_ordering() {
        // The cards Card::from(0..52) preserve the ordering relationship of the
        // underlying numbers.  In other words, Cards and 0..52 are order
        // isomorphic.
        for i in 0..52 {
            let card = Card::from(i);
            // We don't need to do a lower numbers check because of this.
            for hi in (i + 1)..52 {
                let hi_card = Card::from(hi);
                // TEST: Higher numbers lead to higher cards.
                assert!(card < hi_card, "{card} < {hi_card}");
                assert!(hi_card > card, "{hi_card} > {card}");
            }
        }
    }

    #[test]
    fn deck_irrelevance() {
        // We'll do a similar check as natural_ordering, but we'll be using
        // cards from the 1st deck rather than the 0th deck.
        for i in 0..52 {
            let j = i + 52;
            let card_0 = Card::from(i);
            let card_1 = Card::from(j);

            // TEST: Two numbers equivalent mod 52 are "equivalent" cards under
            // Card::from.
            assert_eq!(card_0, card_1);

            // We don't need to do a lower numbers check because of this.
            for hi in (i + 1)..52 {
                let hi_card_0 = Card::from(hi); // deck 0
                let hi_card_1 = Card::from(hi + 52); // deck 1

                // TEST: Higher cards are still ordered better than another
                // deck's cards.
                assert!(card_1 < hi_card_0, "{card_1} < {hi_card_0}");
                assert!(card_0 < hi_card_1, "{card_0} < {hi_card_1}");
                assert!(hi_card_0 > card_1, "{hi_card_0} > {card_1}");
                assert!(hi_card_1 > card_0, "{hi_card_1} > {card_0}");
            }
        }
    }
}

#[cfg(test)]
mod test_impls {
    use std::collections::{HashMap, HashSet};

    use crate::{
        card::{Card, PlayingCard, Rank, Suit},
        helper::ExactSizedArr,
    };

    #[test]
    fn rank() {
        let ranks = Rank::iter_all().collect::<HashSet<_>>();
        // TEST: Rank::iter_all produces all 13 unique ranks.
        assert_eq!(ranks.len(), 13);

        for rank in Rank::iter_all() {
            let cards = rank.cards().collect::<HashSet<_>>();
            assert_eq!(cards.len(), 4, "Expected 4 cards per rank");
            for c in cards {
                // TEST: rank.cards() generates Playing Cards of the same rank
                // as the input in deck 0.
                assert!(matches!(c, Card::PlayingCard(_)));
                let Card::PlayingCard(c) = c else {
                    unreachable!("See above");
                };
                assert_eq!(c.deck, 0);
                assert_eq!(c.rank, rank);
            }

            // Since there are 4 unique cards generated by rank.cards(), and
            // they all have the same deck (0) and rank (rank), they must differ
            // by Suit by virtue of ord.  4 suits suggests (by pigeonhole
            // principle) that all suits must have been used.

            // So rank.cards() generates all cards of deck 0 that have that rank
            // (which is precisely 4 cards).  QED.
        }
    }

    #[test]
    fn suit() {
        let suits = Suit::iter_all().collect::<HashSet<_>>();
        // TEST: Suit::iter_all produces all 4 unique suits.
        assert_eq!(suits.len(), 4);

        for suit in Suit::iter_all() {
            let cards = suit.cards().collect::<HashSet<_>>();
            assert_eq!(cards.len(), 13, "Expected 13 cards per suit");

            for c in cards {
                // TEST: suit.cards() generates Playing Cards of the same suit
                // as the input in deck 0.
                assert!(matches!(c, Card::PlayingCard(_)));
                let Card::PlayingCard(c) = c else {
                    unreachable!("See above");
                };
                assert_eq!(c.deck, 0);
                assert_eq!(c.suit, suit);
            }

            // Similar to rank, pigeonhole principle suggests we must have all
            // 13 ranks of cards in the suit expected for suit.cards().
        }
    }

    #[test]
    fn playing_card() {
        for deck in 0..10 {
            let playing_cards =
                PlayingCard::iter_all(deck).collect::<HashSet<_>>();

            // TEST: Expected 52 cards to be generated by PlayingCard::iter_all.
            assert_eq!(
                playing_cards.len(),
                52,
                "Expected 52 cards in a playing card deck"
            );

            for card in PlayingCard::iter_all(deck)
                .into_array::<52>()
                .unwrap_or_else(|_| {
                    unreachable!(
                        "Look at previous assertion; there must be 52 cards in the iterator."
                    )
                }) {
                // TEST: card.deck must match the input deck
                assert_eq!(card.deck, deck);
                let numeral = i64::from(card);

                // TEST: Expect the card from playing_cards to be bounded by the
                // limits generated by i64::from.
                assert!(
                    numeral >= 52 * deck,
                    "Expected i64::from({}) to be bounded below by {}",
                    card,
                    52 * deck
                );

                assert!(
                    numeral <= (52 * deck) + 51,
                    "Expected i64::from({}) to be bounded above by {}",
                    card,
                    (52 * deck) + 51
                );
            }
        }
    }

    #[test]
    fn card() {
        for decks in 1..10 {
            let cards = Card::iter_all(decks).collect::<Vec<_>>();

            // TEST: Expected there to be `decks` number of decks of cards (2
            // jokers + 52 playing cards) present in Card::iter_all(decks).
            assert_eq!(
                cards.len(),
                (54 * decks) as usize,
                "Expected {} cards in a playing card deck",
                54 * decks,
            );

            // TEST: Expect there to be 2 jokers per deck of cards input.
            assert_eq!(
                cards.iter().filter(|n| matches!(n, Card::Joker(_))).count(),
                (2 * decks) as usize,
                "Expected there to be {} jokers in Card::iter_all({})",
                2 * decks,
                decks,
            );

            // Construct a counter which maps each unique (rank, suit)
            // combination to the count of that combination in cards.
            let counter = {
                let mut counter: HashMap<(Rank, Suit), i64> = HashMap::new();
                for card in &cards {
                    let Card::PlayingCard(card) = card else {
                        continue;
                    };

                    if let Some(count) =
                        counter.get_mut(&(card.rank, card.suit))
                    {
                        *count += 1;
                    } else {
                        counter.insert((card.rank, card.suit), 1);
                    }
                }
                counter
            };

            for (rank, suit) in Rank::iter_all()
                .flat_map(|r| Suit::iter_all().map(move |s| (r, s)))
            {
                // TEST: We expect `decks` instances of a (rank, suit)
                // combination in Card::iter_all(decks).
                assert_eq!(
                    counter[&(rank, suit)],
                    decks,
                    "{} of {} doesn't have {} copies in Card::iter_all({})",
                    rank,
                    suit,
                    decks,
                    decks
                );
            }
        }
    }
}
