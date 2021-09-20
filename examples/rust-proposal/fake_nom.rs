use crate::fake_std::ops::RangeBounds;

pub fn many<H>(range: H)
where
    H: RangeBounds<usize>,
{
    dbg!((range.start_bound(), range.end_bound()));
}
