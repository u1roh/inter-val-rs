mod impl_range_bounds {
    use crate::{Exclusive, Inclusive, LeftBounded, RightBounded};
    use std::ops::{Bound, RangeBounds};

    impl<T: PartialOrd> RangeBounds<T> for LeftBounded<T, Inclusive> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Included(&self.limit)
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
    }
    impl<T: PartialOrd> RangeBounds<T> for LeftBounded<T, Exclusive> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Excluded(&self.limit)
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
    }
    impl<T: PartialOrd> RangeBounds<T> for RightBounded<T, Inclusive> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Included(&self.limit)
        }
    }
    impl<T: PartialOrd> RangeBounds<T> for RightBounded<T, Exclusive> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Excluded(&self.limit)
        }
    }
}

mod converters {
    use crate::{Exclusive, Inclusive, Interval, IntervalIsEmpty};

    /// ```
    /// use std::any::{Any, TypeId};
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a: Interval<_, _, _> = (2..4).try_into().unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Inclusive, Exclusive>>());
    /// assert_eq!(a.left().limit, 2);
    /// assert_eq!(a.right().limit, 4);
    /// ```
    impl<T: PartialOrd> TryFrom<std::ops::Range<T>> for Interval<T, Inclusive, Exclusive> {
        type Error = IntervalIsEmpty;
        fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
            Self::new(r.start.into(), r.end.into()).ok_or(IntervalIsEmpty)
        }
    }

    /// ```
    /// use std::any::{Any, TypeId};
    /// use kd_interval::{Interval, Inclusive};
    /// let a: Interval<_, _, _> = (2..=4).try_into().unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Inclusive, Inclusive>>());
    /// assert_eq!(a.left().limit, 2);
    /// assert_eq!(a.right().limit, 4);
    /// ```
    impl<T: PartialOrd> TryFrom<std::ops::RangeInclusive<T>> for Interval<T, Inclusive> {
        type Error = IntervalIsEmpty;
        fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
            let (left, right) = r.into_inner();
            Self::new(left.into(), right.into()).ok_or(IntervalIsEmpty)
        }
    }

    // /// ```
    // /// use std::any::{Any, TypeId};
    // /// use kd_interval::{IntervalF, Inclusive, Exclusive};
    // /// let a: IntervalF<_, _, _> = (2.74..4.26).try_into().unwrap();
    // /// assert_eq!(a.type_id(), TypeId::of::<IntervalF<f64, Inclusive, Exclusive>>());
    // /// assert_eq!(a.left().limit, 2.74);
    // /// assert_eq!(a.right().limit, 4.26);
    // /// ```
    // impl<T: FloatCore> TryFrom<std::ops::Range<T>> for Interval<T, Inclusive, Exclusive> {
    //     type Error = crate::Error;
    //     fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
    //         Self::try_new(Inclusive.at(r.start), Exclusive.at(r.end))?.ok_or(IntervalIsEmpty.into())
    //     }
    // }

    // /// ```
    // /// use std::any::{Any, TypeId};
    // /// use kd_interval::{IntervalF, Inclusive, Exclusive};
    // /// let a: IntervalF<_, _, _> = (2.74..=4.26).try_into().unwrap();
    // /// assert_eq!(a.type_id(), TypeId::of::<IntervalF<f64, Inclusive, Inclusive>>());
    // /// assert_eq!(a.left().limit, 2.74);
    // /// assert_eq!(a.right().limit, 4.26);
    // /// ```
    // impl<T: FloatCore> TryFrom<std::ops::RangeInclusive<T>> for Interval<T, Inclusive> {
    //     type Error = crate::Error;
    //     fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
    //         let (left, right) = r.into_inner();
    //         Self::try_new(Inclusive.at(left), Inclusive.at(right))?.ok_or(IntervalIsEmpty.into())
    //     }
    // }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let src = Inclusive.at(0).to(Exclusive.at(10)).unwrap();
    /// let dst: std::ops::Range<i32> = src.into();
    /// assert_eq!(dst.start, 0);
    /// assert_eq!(dst.end, 10);
    /// ```
    impl<T> From<Interval<T, Inclusive, Exclusive>> for std::ops::Range<T> {
        fn from(i: Interval<T, Inclusive, Exclusive>) -> Self {
            i.left.0.limit..i.right.0.limit
        }
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive};
    /// let src = Inclusive.at(0).to(Inclusive.at(10)).unwrap();
    /// let dst: std::ops::RangeInclusive<i32> = src.into();
    /// assert_eq!(dst.start(), &0);
    /// assert_eq!(dst.end(), &10);
    /// ```
    impl<T> From<Interval<T, Inclusive, Inclusive>> for std::ops::RangeInclusive<T> {
        fn from(i: Interval<T, Inclusive, Inclusive>) -> Self {
            i.left.0.limit..=i.right.0.limit
        }
    }
}
