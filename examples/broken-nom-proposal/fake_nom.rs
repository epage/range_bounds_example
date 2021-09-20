use std::ops::{Bound, RangeBounds};

pub trait IntoRangeBounds {
    /// Convert to a RangeBound
    fn convert(self) -> (Bound<usize>, Bound<usize>);
}

impl<T> IntoRangeBounds for T
where
    T: RangeBounds<usize>,
{
    fn convert(self) -> (Bound<usize>, Bound<usize>) {
        (self.start_bound().cloned(), self.end_bound().cloned())
    }
}

impl IntoRangeBounds for usize {
    fn convert(self) -> (Bound<usize>, Bound<usize>) {
        (self..=self).convert()
    }
}

pub fn many<G>(range: G)
where
    G: IntoRangeBounds,
{
    let range = range.convert();
    dbg!((range.start_bound(), range.end_bound()));
}
