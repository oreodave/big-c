pub fn ordered<T: Ord, const N: usize>(mut xs: [T; N]) -> [T; N] {
    xs.sort();
    xs
}
