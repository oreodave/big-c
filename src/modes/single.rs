use crate::card::Card;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Single(Card);

impl Single {
    fn new(c: Card) -> Option<Single> {
        (!c.is_joker()).then_some(Single(c))
    }
}

use crate::helper::ordered;
use crate::modes::{Footstool, Hand};

impl Hand for Single {
    fn footstool(&self, other: Self) -> Footstool {
        let self_abs = self.0.deck_abs();
        let other_abs = other.0.deck_abs();

        if self_abs == other_abs {
            Footstool::Full
        } else if self_abs == other_abs + 1 {
            Footstool::Half
        } else {
            Footstool::None
        }
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for Single {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Single({})", self.0)
    }
}
