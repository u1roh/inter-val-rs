mod impl_range_bounds {
    use crate::{Exclusive, Inclusive, LeftBounded, RightBounded};
    use std::ops::{Bound, RangeBounds};

    impl<T: Ord> RangeBounds<T> for LeftBounded<T, Inclusive> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Included(&self.val)
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
    }
    impl<T: Ord> RangeBounds<T> for LeftBounded<T, Exclusive> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Excluded(&self.val)
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
    }
    impl<T: Ord> RangeBounds<T> for RightBounded<T, Inclusive> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Included(&self.val)
        }
    }
    impl<T: Ord> RangeBounds<T> for RightBounded<T, Exclusive> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Excluded(&self.val)
        }
    }
}

mod converters {
    use crate::{Exclusive, Inclusive, Interval};
    use ordered_float::{FloatCore, NotNan};

    // use crate::traits::ScalarFrom;
    //
    // impl<T, U: ScalarFrom<T>> TryFrom<std::ops::Range<T>> for Interval<U, Inclusive, Exclusive>
    // where
    //     crate::Error: From<U::Error>,
    // {
    //     type Error = crate::Error;
    //     fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
    //         Self::new(r.start.into(), r.end.into())
    //     }
    // }
    //
    // impl<T, U: ScalarFrom<T>> TryFrom<std::ops::RangeInclusive<T>> for Interval<U, Inclusive>
    // where
    //     crate::Error: From<U::Error>,
    // {
    //     type Error = crate::Error;
    //     fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
    //         let (left, right) = r.into_inner();
    //         Self::new(left.into(), right.into())
    //     }
    // }

    impl<T: Ord> TryFrom<std::ops::Range<T>> for Interval<T, Inclusive, Exclusive> {
        type Error = crate::IntervalIsEmpty;
        fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
            Self::new(r.start.into(), r.end.into())
        }
    }
    impl<T: Ord> TryFrom<std::ops::RangeInclusive<T>> for Interval<T, Inclusive> {
        type Error = crate::IntervalIsEmpty;
        fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
            let (left, right) = r.into_inner();
            Self::new(left.into(), right.into())
        }
    }

    impl<T: FloatCore> TryFrom<std::ops::Range<T>> for Interval<NotNan<T>, Inclusive, Exclusive> {
        type Error = crate::Error;
        fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
            Self::try_new(Inclusive.at(r.start), Exclusive.at(r.end))
        }
    }
    impl<T: FloatCore> TryFrom<std::ops::RangeInclusive<T>> for Interval<NotNan<T>, Inclusive> {
        type Error = crate::Error;
        fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
            let (left, right) = r.into_inner();
            Self::try_new(Inclusive.at(left), Inclusive.at(right))
        }
    }
}
