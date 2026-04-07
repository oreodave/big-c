use crate::card::{Card, PlayingCard, Rank, Suit};
use std::convert::TryFrom;

/*
(1) In C this would just be a straight cast lol.  At least T -> i64 is a normal
cast.
 */
impl TryFrom<i64> for Rank {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Three),
            1 => Ok(Self::Four),
            2 => Ok(Self::Five),
            3 => Ok(Self::Six),
            4 => Ok(Self::Seven),
            5 => Ok(Self::Eight),
            6 => Ok(Self::Nine),
            7 => Ok(Self::Ten),
            8 => Ok(Self::Jack),
            9 => Ok(Self::Queen),
            10 => Ok(Self::King),
            11 => Ok(Self::Ace),
            12 => Ok(Self::Two),
            _ => Err(()),
        }
    }
}

// (1)
impl TryFrom<i64> for Suit {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Diamond),
            1 => Ok(Self::Club),
            2 => Ok(Self::Heart),
            3 => Ok(Self::Spade),
            _ => Err(()),
        }
    }
}

impl TryFrom<i64> for PlayingCard {
    type Error = ();

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        if n >= 0 {
            let deck = n / 52;
            let n = n % 52;
            // NOTE: If only Rust had Ada-like numeric contracts, this wouldn't
            // be necessary; n >= 0 => n % 52 in [0, 51] so Rank::try_from and
            // Suit::try_from will always succeed.
            let rank = Rank::try_from(n / 4).unwrap();
            let suit = Suit::try_from(n % 4).unwrap();
            Ok(Self { deck, rank, suit })
        } else {
            Err(())
        }
    }
}

impl From<PlayingCard> for i64 {
    fn from(card: PlayingCard) -> i64 {
        let deck = card.deck;
        let rank = card.rank as i64;
        let suit = card.suit as i64;
        (deck * 52) + (rank * 4) + suit
    }
}

impl From<i64> for Card {
    fn from(n: i64) -> Self {
        if n < 0 {
            Self::Joker(n)
        } else {
            // Since n >= 0, this should always succeed
            PlayingCard::try_from(n).map(Self::PlayingCard).unwrap()
        }
    }
}

impl From<Card> for i64 {
    fn from(card: Card) -> i64 {
        match card {
            Card::Joker(x) => x,
            Card::PlayingCard(pc) => i64::from(pc),
        }
    }
}
