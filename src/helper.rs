/// Given an array of arguments, return them sorted.  Best utilised with array
/// destructuring.
pub fn ordered<T: Ord, const N: usize>(mut xs: [T; N]) -> [T; N] {
    xs.sort();
    xs
}

/// An iterator adaptor (derived from ExactSizedIterator) which has a guaranteed
/// compile time size, allowing for collection of an iterator into a stack
/// allocated array.
pub trait ExactSizedArr<I>: ExactSizeIterator<Item = I> + Sized
where
    I: Default,
{
    fn into_array<const N: usize>(mut self) -> Result<[Self::Item; N], Self> {
        if self.len() < N {
            Err(self)
        } else {
            Ok(std::array::from_fn(|_| self.next().unwrap()))
        }
    }
}

/// Default implementation of ExactSizedArr for any ExactSizeIterator.
impl<T, I> ExactSizedArr<T> for I
where
    T: Default + Copy + Clone,
    I: ExactSizeIterator<Item = T>,
{
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
