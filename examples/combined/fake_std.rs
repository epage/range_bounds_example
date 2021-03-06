pub mod cmp {
    pub trait Ord {}

    impl Ord for usize {}
}

pub mod iter {
    pub trait Step: Clone + PartialOrd<Self> {
        fn forward(start: Self, count: usize) -> Self;
        fn backward(start: Self, count: usize) -> Self;
    }

    impl Step for usize {
        fn forward(start: Self, count: usize) -> Self {
            start + count
        }
        fn backward(start: Self, count: usize) -> Self {
            start - count
        }
    }
}

pub mod ops {
    pub use std::ops::{
        Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
    };

    /// `RangeBounds` is implemented by Rust's built-in range types, produced
    /// by range syntax like `..`, `a..`, `..b`, `..=c`, `d..e`, or `f..=g`.
    pub trait RangeBounds<T: ?Sized> {
        /// Start index bound.
        ///
        /// Returns the start value as a `Bound`.
        ///
        /// # Examples
        ///
        /// ```
        /// # fn main() {
        /// use std::ops::Bound::*;
        /// use std::ops::RangeBounds;
        ///
        /// assert_eq!((..10).start_bound(), Unbounded);
        /// assert_eq!((3..10).start_bound(), Included(&3));
        /// # }
        /// ```
        fn start_bound(&self) -> Bound<&T>;

        /// End index bound.
        ///
        /// Returns the end value as a `Bound`.
        ///
        /// # Examples
        ///
        /// ```
        /// # fn main() {
        /// use std::ops::Bound::*;
        /// use std::ops::RangeBounds;
        ///
        /// assert_eq!((3..).end_bound(), Unbounded);
        /// assert_eq!((3..10).end_bound(), Excluded(&10));
        /// # }
        /// ```
        fn end_bound(&self) -> Bound<&T>;

        /// Returns `true` if `item` is contained in the range.
        ///
        /// # Examples
        ///
        /// ```
        /// assert!( (3..5).contains(&4));
        /// assert!(!(3..5).contains(&2));
        ///
        /// assert!( (0.0..1.0).contains(&0.5));
        /// assert!(!(0.0..1.0).contains(&f32::NAN));
        /// assert!(!(0.0..f32::NAN).contains(&0.5));
        /// assert!(!(f32::NAN..1.0).contains(&0.5));
        fn contains<U>(&self, item: &U) -> bool
        where
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>,
        {
            (match self.start_bound() {
                Included(ref start) => *start <= item,
                Excluded(ref start) => *start < item,
                Unbounded => true,
            }) && (match self.end_bound() {
                Included(ref end) => item <= *end,
                Excluded(ref end) => item < *end,
                Unbounded => true,
            })
        }
    }

    use self::Bound::{Excluded, Included, Unbounded};

    impl<T: ?Sized> RangeBounds<T> for RangeFull {
        fn start_bound(&self) -> Bound<&T> {
            Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Unbounded
        }
    }

    impl<T> RangeBounds<T> for RangeFrom<T> {
        fn start_bound(&self) -> Bound<&T> {
            Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&T> {
            Unbounded
        }
    }

    impl<T> RangeBounds<T> for RangeTo<T> {
        fn start_bound(&self) -> Bound<&T> {
            Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Excluded(&self.end)
        }
    }

    impl<T> RangeBounds<T> for Range<T> {
        fn start_bound(&self) -> Bound<&T> {
            Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&T> {
            Excluded(&self.end)
        }
    }

    impl<T> RangeBounds<T> for RangeInclusive<T> {
        fn start_bound(&self) -> Bound<&T> {
            Included(self.start())
        }
        fn end_bound(&self) -> Bound<&T> {
            Included(self.end())
        }
    }

    impl<T> RangeBounds<T> for RangeToInclusive<T> {
        fn start_bound(&self) -> Bound<&T> {
            Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Included(&self.end)
        }
    }

    impl<T> RangeBounds<T> for (Bound<T>, Bound<T>) {
        fn start_bound(&self) -> Bound<&T> {
            match *self {
                (Included(ref start), _) => Included(start),
                (Excluded(ref start), _) => Excluded(start),
                (Unbounded, _) => Unbounded,
            }
        }

        fn end_bound(&self) -> Bound<&T> {
            match *self {
                (_, Included(ref end)) => Included(end),
                (_, Excluded(ref end)) => Excluded(end),
                (_, Unbounded) => Unbounded,
            }
        }
    }

    impl<'a, T: ?Sized + 'a> RangeBounds<T> for (Bound<&'a T>, Bound<&'a T>) {
        fn start_bound(&self) -> Bound<&T> {
            self.0
        }

        fn end_bound(&self) -> Bound<&T> {
            self.1
        }
    }

    impl<T> RangeBounds<T> for RangeFrom<&T> {
        fn start_bound(&self) -> Bound<&T> {
            Included(self.start)
        }
        fn end_bound(&self) -> Bound<&T> {
            Unbounded
        }
    }

    impl<T> RangeBounds<T> for RangeTo<&T> {
        fn start_bound(&self) -> Bound<&T> {
            Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Excluded(self.end)
        }
    }

    impl<T> RangeBounds<T> for Range<&T> {
        fn start_bound(&self) -> Bound<&T> {
            Included(self.start)
        }
        fn end_bound(&self) -> Bound<&T> {
            Excluded(self.end)
        }
    }

    impl<T> RangeBounds<T> for RangeInclusive<&T> {
        fn start_bound(&self) -> Bound<&T> {
            Included(self.start())
        }
        fn end_bound(&self) -> Bound<&T> {
            Included(self.end())
        }
    }

    impl<T> RangeBounds<T> for RangeToInclusive<&T> {
        fn start_bound(&self) -> Bound<&T> {
            Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Included(self.end)
        }
    }

    macro_rules! range_bounds_scalar {
        ($ty:ty) => {
            impl RangeBounds<$ty> for $ty {
                fn start_bound(&self) -> Bound<&$ty> {
                    Included(&self)
                }
                fn end_bound(&self) -> Bound<&$ty> {
                    Included(&self)
                }
            }
        };
    }

    range_bounds_scalar!(char);
    range_bounds_scalar!(i8);
    range_bounds_scalar!(i16);
    range_bounds_scalar!(i32);
    range_bounds_scalar!(i64);
    range_bounds_scalar!(i128);
    range_bounds_scalar!(isize);
    range_bounds_scalar!(u8);
    range_bounds_scalar!(u16);
    range_bounds_scalar!(u32);
    range_bounds_scalar!(u64);
    range_bounds_scalar!(u128);
    range_bounds_scalar!(usize);
}
