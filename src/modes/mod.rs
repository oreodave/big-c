pub mod pair;
pub mod single;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Footstool {
    None,
    Half,
    Full,
}

pub trait Hand {
    /** Given two instances of a Hand (`self` and `other`), verify if `self`
    footstools `other`.
     */
    fn footstool(&self, other: &Self) -> Footstool;

    // Only need to implement is_proper.
    fn is_proper(&self) -> bool;

    fn is_improper(&self) -> bool {
        !self.is_proper()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /** Given two hands, assert that their footstool condition is non-reflexive.
    Return the results of the two footstool checks (x on y, y on x).
     */
    pub fn test_footstool_non_reflexivity<T: Hand + Copy>(
        x: &T,
        y: &T,
    ) -> (Footstool, Footstool) {
        let res1 = x.footstool(y);
        let res2 = y.footstool(x);
        matches!(
            (res1, res2),
            (Footstool::None, Footstool::None)
                | (Footstool::None, Footstool::Half)
                | (Footstool::Half, Footstool::None)
                | (Footstool::Full, Footstool::Full)
        );
        (res1, res2)
    }
}
