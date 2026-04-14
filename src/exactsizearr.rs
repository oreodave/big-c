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
