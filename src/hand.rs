use crate::Card;

#[derive(Debug)]
pub enum PokerType {
    TwoPair,
    Flush,
    Straight,
    FullHouse,
    FourKind,
    FiveKind,
    StraightFlush,
}

#[derive(Debug)]
pub enum Hand {
    Single(Card),
    Pair(Card, Card),
    Triple(Card, Card, Card),
    Poker {
        poker_type: PokerType,
        c1: Card,
        c2: Card,
        c3: Card,
        c4: Card,
        c5: Card,
    },
}
