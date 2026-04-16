use crate::card::Card;

pub mod item;
pub mod pair;
pub mod single;
pub mod triple;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Footstool {
    None,
    Half,
    Full,
}

pub trait Hand: Ord {
    /// Return true if the current hand doesn't have any jokers.
    fn is_proper(&self) -> bool;

    /// Return true if the current hand has at least one Joker.
    fn is_improper(&self) -> bool {
        !self.is_proper()
    }

    /// Get the high card of the current hand.
    fn high_card(&self) -> Card;

    /// Given two instances of a Hand (`self` and `other`), return if `self`
    /// footstools `other`.
    fn footstool(&self, other: &Self) -> Footstool;
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display};

    use super::*;

    /// Given two hands, assert that applying a footstool both ways fits a
    /// recognised basic pattern for footstools (in a generic sense).  Return
    /// the results of the two footstool checks (x on y, y on x).
    ///
    /// Obviously may panic.
    pub fn test_footstool<T>(x: &T, y: &T) -> (Footstool, Footstool)
    where
        T: Hand + Copy + Display + Debug,
    {
        let res1 = x.footstool(y);
        let res2 = y.footstool(x);
        assert!(
            match (res1, res2) {
                (Footstool::None, Footstool::None) => true,
                (Footstool::Half, Footstool::None) => x > y,
                (Footstool::None, Footstool::Half) => y > x,
                (Footstool::Full, Footstool::Full) => x == y,
                _ => false,
            },
            "Expected footstool on {}, {} ({:?}, {:?}) to match a recognised pattern",
            x,
            y,
            res1,
            res2
        );
        (res1, res2)
    }
}
