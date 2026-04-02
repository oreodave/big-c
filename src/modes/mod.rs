mod pair;
mod single;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Footstool {
    None,
    Half,
    Full,
}

pub trait Hand {
    fn footstool(&self, other: Self) -> Footstool;
}
