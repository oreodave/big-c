/// An iterator adaptor (derived from ExactSizedIterator) which has a guaranteed
/// compile time size, allowing for collection of an iterator into a stack
/// allocated array.
pub trait ExactSizedArr<I>: ExactSizeIterator + Sized {
    fn into_array<const N: usize>(mut self) -> Result<[Self::Item; N], Self> {
        if self.len() < N {
            Err(self)
        } else {
            Ok(std::array::from_fn(|_| self.next().unwrap()))
        }
    }
}

/// Default implementation of ExactSizedArr for any ExactSizeIterator.
impl<I> ExactSizedArr<I> for I where I: ExactSizeIterator {}
