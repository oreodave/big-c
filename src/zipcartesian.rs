use std::iter::Iterator;

pub trait ZipCatersianExt: Iterator {
    fn zip_cartesian<B>(
        self,
        b: B,
    ) -> impl Iterator<Item = (Self::Item, B::Item)> + Clone
    where
        Self::Item: Copy,
        Self: Sized + Clone,
        B: Iterator<Item: Copy> + Clone;
}

impl<I: Iterator<Item: Copy> + Clone> ZipCatersianExt for I {
    /// Exhaustive coupling of two iterators.
    /// For each x in `self`: for each y in `b`: yield (x, y).
    /// b: B must implement `Clone`.
    fn zip_cartesian<B: Iterator<Item: Copy> + Clone>(
        self,
        b: B,
    ) -> impl Iterator<Item = (Self::Item, B::Item)> + Clone {
        self.flat_map(move |a_item| {
            b.clone().map(move |b_item| (a_item, b_item))
        })
    }
}
