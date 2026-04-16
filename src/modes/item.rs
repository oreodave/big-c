use crate::{
    card::Card,
    modes::{pair::Pair, single::Single, triple::Triple},
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// An Item is a validated cardset, variant over all different modes.
pub enum Item {
    Single(Single),
    Pair(Pair),
    Triple(Triple),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Possible errors from attempting to generate a new Item from a set of cards.
pub enum ItemParseError {
    InvalidSingle(Card),
    InvalidPair(Card, Card),
    InvalidTriple(Card, Card, Card),
    InvalidArity,
}

impl Item {
    pub fn parse(cards: &[Card]) -> Result<Item, ItemParseError> {
        match cards {
            [a] => Single::new(*a)
                .map(Self::Single)
                .ok_or(ItemParseError::InvalidSingle(*a)),
            [a, b] => Pair::new(*a, *b)
                .map(Self::Pair)
                .ok_or(ItemParseError::InvalidPair(*a, *b)),
            [a, b, c] => Triple::new(*a, *b, *c)
                .map(Self::Triple)
                .ok_or(ItemParseError::InvalidTriple(*a, *b, *c)),
            _ => Err(ItemParseError::InvalidArity),
        }
    }
}

use std::fmt::{self, Display, Formatter};

impl Display for ItemParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSingle(a) => write!(f, "invalid single {}", a),
            Self::InvalidPair(a, b) => write!(f, "invalid pair {} + {}", a, b),
            Self::InvalidTriple(a, b, c) => {
                write!(f, "invalid triple {} + {} + {}", a, b, c)
            }
            Self::InvalidArity => write!(f, "invalid arity"),
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single(x) => write!(f, "{}", x),
            Self::Pair(x) => write!(f, "{}", x),
            Self::Triple(x) => write!(f, "{}", x),
        }
    }
}
