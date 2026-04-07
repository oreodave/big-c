use crate::{
    card::{Card, PlayingCard},
    helper::ordered,
};

#[derive(Debug, Copy, Clone)]
pub struct Triple(Card, Card, Card);

impl Triple {
    fn new(c1: Card, c2: Card, c3: Card) -> Option<Triple> {
        let [c1, c2, c3] = ordered([c1, c2, c3]);

        match (c1, c2, c3) {
            (Card::Joker(_), Card::Joker(_), Card::Joker(_)) => None,
            // NOTE: c3 should not be a joker now.
            (Card::Joker(_), Card::Joker(_), _) => Some(Triple(c1, c2, c3)),
            (
                Card::Joker(_),
                Card::PlayingCard(PlayingCard { rank: r1, .. }),
                Card::PlayingCard(PlayingCard { rank: r2, .. }),
            ) if r1 == r2 => Some(Triple(c1, c2, c3)),

            (
                Card::PlayingCard(PlayingCard { rank: r1, .. }),
                Card::PlayingCard(PlayingCard { rank: r2, .. }),
                Card::PlayingCard(PlayingCard { rank: r3, .. }),
            ) if r1 == r2 && r2 == r3 => Some(Triple(c1, c2, c3)),

            _ => None,
        }
    }
}
