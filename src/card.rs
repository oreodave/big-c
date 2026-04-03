#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Rank {
    Three = 0,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Two,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Suit {
    Diamond = 0,
    Club,
    Heart,
    Spade,
}

#[derive(Eq, Debug, Clone, Copy)]
pub struct PlayingCard {
    pub deck: i64,
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Eq, Debug, Clone, Copy)]
pub enum Card {
    Joker(i64),
    PlayingCard(PlayingCard),
}

impl Rank {
    pub fn ordinary_order(&self) -> i32 {
        ((*self as i32) + 2) % 13
    }

    /** Generate an iterator over all ranks. */
    pub fn iter_all() -> impl Iterator<Item = Rank> {
        (0..13).filter_map(|x| Rank::try_from(x).ok())
    }

    /** Generate an iterator over all cards within a rank, ordered by Suit. */
    pub fn cards(&self) -> impl Iterator<Item = Card> {
        let n = *self as i64;
        ((n * 4)..((n + 1) * 4)).map(Card::from)
    }
}

impl Suit {
    /** Generate an iterator over all suits. */
    pub fn iter_all() -> impl Iterator<Item = Suit> {
        (0..4).filter_map(|x| Suit::try_from(x).ok())
    }

    /** Generate an iterator over all cards within a suit, ordered by Suit. */
    pub fn cards(&self) -> impl Iterator<Item = Card> {
        Rank::iter_all().map(|rank| Card::make_playing_card(rank, *self))
    }
}

impl PlayingCard {
    pub fn new(deck: i64, rank: Rank, suit: Suit) -> Self {
        Self { deck, rank, suit }
    }

    pub fn abs(&self) -> i64 {
        let rank = self.rank as i64;
        let suit = self.suit as i64;
        (rank * 4) + suit
    }
}

impl Card {
    pub fn make_joker() -> Self {
        Self::Joker(-1)
    }

    pub fn make_playing_card(rank: Rank, suit: Suit) -> Self {
        Self::PlayingCard(PlayingCard::new(0, rank, suit))
    }

    pub fn deck_abs(&self) -> i64 {
        match *self {
            Self::Joker(x) => x,
            Self::PlayingCard(card) => card.abs(),
        }
    }

    pub fn is_joker(&self) -> bool {
        matches!(self, Self::Joker(_))
    }

    pub fn playing_card(&self) -> Option<PlayingCard> {
        match *self {
            Self::Joker(_) => None,
            Self::PlayingCard(card) => Some(card),
        }
    }

    pub fn rank(&self) -> Option<Rank> {
        self.playing_card().map(|pc| pc.rank)
    }

    pub fn suit(&self) -> Option<Suit> {
        self.playing_card().map(|pc| pc.suit)
    }
}

/** Given a sequence of Playing Cards, check if they are all of the same rank.
 */
pub fn all_same_rank(cards: &[PlayingCard]) -> bool {
    let rank = cards[0].rank;
    cards[1..].iter().all(|card| rank == card.rank)
}

/** Generate a vector of cards representing the concatenation of
`number_of_decks` decks of playing cards put together.

Note that each deck gets two jokers - this is added to the overall vector.
 */
pub fn make_decks(number_of_decks: usize) -> Vec<Card> {
    let number_of_decks: i64 = number_of_decks.try_into().unwrap();
    (-(number_of_decks * 2)..(52 * number_of_decks))
        .map(Card::from)
        .collect::<Vec<Card>>()
}

mod trait_display {
    use super::*;
    use std::fmt::{Display, Formatter};

    impl Display for Rank {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Rank::Jack => "J",
                    Rank::Queen => "Q",
                    Rank::King => "K",
                    Rank::Ace => "A",
                    Rank::Two => "2",
                    Rank::Three => "3",
                    Rank::Four => "4",
                    Rank::Five => "5",
                    Rank::Six => "6",
                    Rank::Seven => "7",
                    Rank::Eight => "8",
                    Rank::Nine => "9",
                    Rank::Ten => "10",
                }
            )
        }
    }

    impl Display for Suit {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Suit::Diamond => "♦",
                    Suit::Club => "♣",
                    Suit::Heart => "♥",
                    Suit::Spade => "♠",
                }
            )
        }
    }

    impl Display for PlayingCard {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}[{}]", self.rank, self.suit)
        }
    }

    impl Display for Card {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Card::Joker(_) => write!(f, "Joker"),
                Card::PlayingCard(card) => write!(f, "{}", card),
            }
        }
    }
}

mod traits_numerics {
    use super::*;
    use std::convert::TryFrom;

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

    impl From<i64> for Card {
        fn from(n: i64) -> Self {
            if n < 0 {
                Self::Joker(n)
            } else {
                // Since n >= 0, this should always succeed
                let pc = PlayingCard::try_from(n).unwrap();
                Self::PlayingCard(pc)
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

    impl From<Card> for i64 {
        fn from(card: Card) -> i64 {
            match card {
                Card::Joker(x) => x,
                Card::PlayingCard(pc) => i64::from(pc),
            }
        }
    }
}

mod traits_ord {
    use super::*;
    use std::cmp::Ordering;

    impl Ord for PlayingCard {
        fn cmp(&self, other: &Self) -> Ordering {
            self.abs().cmp(&other.abs())
        }
    }

    impl Ord for Card {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (Self::PlayingCard(c1), Self::PlayingCard(c2)) => c1.cmp(c2),
                (Self::Joker(_), Self::Joker(_)) => Ordering::Equal,
                (Self::Joker(_), _) => Ordering::Less,
                (_, Self::Joker(_)) => Ordering::Greater,
            }
        }
    }

    impl PartialEq for PlayingCard {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(other) == Ordering::Equal
        }
    }

    impl PartialEq for Card {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(other) == Ordering::Equal
        }
    }

    impl PartialOrd for PlayingCard {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}

mod traits_hash {
    use super::*;
    use std::hash::{Hash, Hasher};

    impl Hash for Card {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // NOTE: We're using the i64 conversion of card for the hash since that
            // should generate unique numbers per card.
            i64::from(*self).hash(state);
        }
    }
}
