mod pair;
mod single;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Footstool {
    None,
    Half,
    Full,
}

pub trait Hand {
    fn footstool(&self, other: &Self) -> Footstool;
}

mod tests {
    use super::*;

    /** Given two hands, assert that their footstool condition is non-reflexive.
     * Return the results of the two footstool checks (x on y, y on x).
     */
    pub fn test_non_reflexivity<T: Hand + Copy>(
        x: &T,
        y: &T,
    ) -> (Footstool, Footstool) {
        let res1 = x.footstool(y);
        let res2 = y.footstool(x);
        assert!(match (res1, res2) {
            (Footstool::None, Footstool::None)
            | (Footstool::None, Footstool::Half)
            | (Footstool::Half, Footstool::None)
            | (Footstool::Full, Footstool::Full) => true,
            _ => false,
        });
        (res1, res2)
    }
}
