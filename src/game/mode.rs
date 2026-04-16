use crate::{
    card::Card,
    modes::item::{Item, ItemParseError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// All possible stages of a Round where an Item may be presented by a player,
/// distinguished by Mode (Single, Pair, Triples, etc).  `Any` represents the
/// "decision" stage where a single player may play any Item of any of the
/// available Round Modes to start off the new round.
pub enum Mode {
    Any,
    Single,
    Pair,
    Triple,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Types of errors which may occur when validating some set of cards against a
/// Mode.
pub enum ModeValidateError {
    ParseError(ItemParseError),
    ModeMismatch,
}

impl Mode {
    /// Given mode `self`, validate if `cards` creates a valid Item for that
    /// mode.
    /// Returns `Ok(Item)` if so, otherwise return `Err(ModeValidateError)`.
    pub fn validate_cards(
        &self,
        cards: &[Card],
    ) -> Result<Item, ModeValidateError> {
        let item = Item::parse(cards).map_err(ModeValidateError::ParseError)?;
        if *self == Mode::Any || *self == item.mode() {
            Ok(item)
        } else {
            Err(ModeValidateError::ModeMismatch)
        }
    }
}

impl Item {
    /// Return the Mode we'd expect this Item to be played in - trivial enum
    /// conversion.
    fn mode(&self) -> Mode {
        match self {
            Self::Single(_) => Mode::Single,
            Self::Pair(_) => Mode::Pair,
            Self::Triple(_) => Mode::Triple,
        }
    }
}
