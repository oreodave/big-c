#[derive(PartialEq, Debug, Copy, Clone)]
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

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Suit {
    Diamond = 0,
    Club,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy)]
pub enum Card {
    Joker(i64),
    PlayingCard { deck: i64, rank: Rank, suit: Suit },
}

impl Rank {
    pub fn ordinary_order(&self) -> i32 {
        ((*self as i32) + 2) % 13
    }
}

impl Card {
    pub fn is_joker(&self) -> bool {
        matches!(self, Self::Joker(_))
    }

    pub fn rank(&self) -> Option<Rank> {
        match self {
            Self::Joker(_) => None,
            Self::PlayingCard { rank, .. } => Some(*rank),
        }
    }

    pub fn suit(&self) -> Option<Suit> {
        match self {
            Self::Joker(_) => None,
            Self::PlayingCard { suit, .. } => Some(*suit),
        }
    }
}

pub fn make_decks(number_of_decks: usize) -> Vec<Card> {
    let number_of_decks: i64 = number_of_decks.try_into().unwrap();
    (-(number_of_decks * 2)..(52 * number_of_decks))
        .map(Card::from)
        .collect::<Vec<Card>>()
}

mod traits {
    use std::cmp::Ordering;
    use std::convert::TryFrom;
    use std::fmt::{Display, Formatter};
    use std::hash::{Hash, Hasher};

    use super::*;

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

    impl Display for Card {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Card::Joker(_) => write!(f, "Joker"),
                Card::PlayingCard { rank, suit, .. } => {
                    write!(f, "{}[{}]", rank, suit)
                }
            }
        }
    }

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

    impl From<i64> for Card {
        fn from(n: i64) -> Card {
            if n < 0 {
                Card::Joker(n)
            } else {
                let deck = n / 52;
                let n = n % 52;
                // NOTE: If only Rust had Ada-like numeric contracts, this wouldn't
                // be necessary; n >= 0 => n % 52 in [0, 51] so Rank::try_from and
                // Suit::try_from will always succeed
                let rank = Rank::try_from(n / 4).unwrap();
                let suit = Suit::try_from(n % 4).unwrap();
                Card::PlayingCard { deck, rank, suit }
            }
        }
    }

    impl From<Card> for i64 {
        fn from(card: Card) -> i64 {
            match card {
                Card::Joker(x) => x,
                Card::PlayingCard { deck, rank, suit } => {
                    (deck * 52) + ((rank as i64) * 4) + (suit as i64)
                }
            }
        }
    }

    impl PartialEq for Card {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(other) == Ordering::Equal
        }
    }

    impl Eq for Card {}

    impl Ord for Card {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.is_joker() && other.is_joker() {
                Ordering::Equal
            } else if self.is_joker() {
                Ordering::Less
            } else if other.is_joker() {
                Ordering::Greater
            } else {
                let self_val = i64::from(*self) % 52;
                let other_val = i64::from(*other) % 52;
                self_val.cmp(&other_val)
            }
        }
    }

    impl Hash for Card {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // NOTE: We're using the i64 conversion of card for the hash since that
            // should generate unique numbers per card.
            i64::from(*self).hash(state);
        }
    }

    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}
