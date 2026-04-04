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
    // Only need to implement is_proper.
    fn is_proper(&self) -> bool;

    fn is_improper(&self) -> bool {
        !self.is_proper()
    }

    /** Given two instances of a Hand (`self` and `other`), verify if `self`
    footstools `other`.
     */
    fn footstool(&self, other: &Self) -> Footstool;
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;

    /** Given two hands, assert that applying a footstool both ways fits a
    recognised pattern.  Return the results of the two footstool checks (x on y,
    y on x).
     */
    pub fn test_footstool<T>(x: &T, y: &T) -> (Footstool, Footstool)
    where
        T: Hand + Copy + Display,
    {
        let res1 = x.footstool(y);
        let res2 = y.footstool(x);
        assert!(
            match (res1, res2) {
                // Default patterns we'd expect
                (Footstool::Half, Footstool::None)
                | (Footstool::None, Footstool::Half)
                | (Footstool::None, Footstool::None)
                    => true,

                // Patterns that require an exact examination to be certain
                (Footstool::Full, Footstool::Full) => x == y,
                _ => true,
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
