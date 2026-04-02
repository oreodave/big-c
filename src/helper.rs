use std::cmp::{max, min};

pub fn ordered<T: Ord + Copy>(x: T, y: T) -> (T, T) {
    (min(x, y), max(x, y))
}
