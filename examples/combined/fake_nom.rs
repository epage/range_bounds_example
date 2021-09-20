use crate::fake_std::ops::{RangeBounds, RangeInclusive};

pub trait IntoRangeBounds<T>
where
    T: RangeBounds<usize>,
{
    /// Convert to a RangeBound
    fn convert(self) -> T;
}

impl<T> IntoRangeBounds<T> for T
where
    T: RangeBounds<usize>,
{
    fn convert(self) -> T {
        self
    }
}

impl IntoRangeBounds<RangeInclusive<usize>> for usize {
    fn convert(self) -> RangeInclusive<usize> {
        self..=self
    }
}

pub fn many<G, H>(range: G)
where
    G: IntoRangeBounds<H>,
    H: RangeBounds<usize>,
{
    let range = range.convert();
    dbg!((range.start_bound(), range.end_bound()));
}
