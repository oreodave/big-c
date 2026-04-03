use crate::{
    card::{Card, PlayingCard},
    helper::ordered,
};

#[derive(Debug, Copy, Clone)]
pub struct Triple(pub Card, pub Card, pub Card);

impl Triple {
    fn new(c1: Card, c2: Card, c3: Card) -> Option<Triple> {
        let [c1, c2, c3] = ordered([c1, c2, c3]);

        match (c1, c2, c3) {
            // Invalid triple if you have 3 jokers
            (Card::Joker(_), Card::Joker(_), Card::Joker(_)) => None,

            // NOTE: c3 cannot be a joker now.  If the other two cards are
            // jokers, then this is fine as a triple.
            (Card::Joker(_), Card::Joker(_), _) => Some(Triple(c1, c2, c3)),

            // NOTE: c2 cannot be a joker now.  If only one card is a joker, the
            // other two card's ranks must match.
            (
                Card::Joker(_),
                Card::PlayingCard(PlayingCard { rank: r2, .. }),
                Card::PlayingCard(PlayingCard { rank: r3, .. }),
            ) if r2 == r3 => Some(Triple(c1, c2, c3)),

            // NOTE: all cards are playing cards.  All ranks must match in order
            // to be a triple.
            (
                Card::PlayingCard(PlayingCard { rank: r1, .. }),
                Card::PlayingCard(PlayingCard { rank: r2, .. }),
                Card::PlayingCard(PlayingCard { rank: r3, .. }),
            ) if r1 == r2 && r2 == r3 => Some(Triple(c1, c2, c3)),

            // Once again, not necessary since the previous patterns should
            // encapsulate all possible combinations due to ordering, but the
            // compiler needs the juice.
            _ => None,
        }
    }
}
