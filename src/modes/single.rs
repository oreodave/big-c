use crate::card::Card;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Single(Card);

impl Single {
    fn new(c: Card) -> Option<Single> {
        (!c.is_joker()).then_some(Single(c))
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for Single {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Single({})", self.0)
    }
}
