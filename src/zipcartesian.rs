use std::iter::Iterator;

pub trait ZipCartesianExt: Iterator + Clone
where
    Self::Item: Copy,
{
    fn zip_cartesian<B>(
        self,
        b: B,
    ) -> impl Iterator<Item = (Self::Item, B::Item)> + Clone
    where
        B::Item: Copy,
        B: Iterator + Clone;
}

impl<I> ZipCartesianExt for I
where
    I: Iterator + Clone,
    I::Item: Copy,
{
    /// Exhaustive coupling of two iterators.
    /// For each x in `self`: for each y in `b`: yield (x, y).
    /// b: B must implement `Clone`.
    fn zip_cartesian<B>(
        self,
        b: B,
    ) -> impl Iterator<Item = (Self::Item, B::Item)> + Clone
    where
        B::Item: Copy,
        B: Iterator + Clone,
    {
        self.flat_map(move |a_item| {
            b.clone().map(move |b_item| (a_item, b_item))
        })
    }
}
