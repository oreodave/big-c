/// Given an array of arguments, return them sorted.  Best utilised with array
/// destructuring.
pub fn ordered<T: Ord, const N: usize>(mut xs: [T; N]) -> [T; N] {
    xs.sort();
    xs
}

/// A macro which generates Eq, PartialEq, and PartialOrd implementations for
/// some given type.  These implementations are dependent on Ord already being
/// implemented for that type.
macro_rules! impl_cmp_eq_on_ord {
    ($type:ident) => {
        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == std::cmp::Ordering::Equal
            }
        }

        impl Eq for $type {}

        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
    };
}

pub(crate) use impl_cmp_eq_on_ord;
