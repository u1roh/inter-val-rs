use crate::{Bound, Exclusive, Inclusive, Interval};

impl From<Inclusive> for Bound {
    fn from(_: Inclusive) -> Self {
        Self::Inclusive
    }
}
impl From<Exclusive> for Bound {
    fn from(_: Exclusive) -> Self {
        Self::Exclusive
    }
}

// impl<T> From<(T, Inclusive)> for (T, Bound) {
//     fn from(b: (T, Inclusive)) -> Self {
//         Self::Inclusive(b.0)
//     }
// }
// impl<T> From<(T, Exclusive)> for (T, Bound) {
//     fn from(b: (T, Exclusive)) -> Self {
//         Self::Exclusive(b.0)
//     }
// }

// impl<T: ordered_float::FloatCore> TryFrom<(T, Inclusive)> for Inclusive<NotNan<T>> {
//     type Error = ordered_float::FloatIsNan;
//     fn try_from(b: (T, Inclusive)) -> Result<Self, Self::Error> {
//         NotNan::new(b.0).map(Self)
//     }
// }
// impl<T: ordered_float::FloatCore> TryFrom<(T, Exclusive)> for Exclusive<NotNan<T>> {
//     type Error = ordered_float::FloatIsNan;
//     fn try_from(b: (T, Exclusive)) -> Result<Self, Self::Error> {
//         NotNan::new(b.0).map(Self)
//     }
// }
// impl<T: ordered_float::FloatCore> TryFrom<(T, Bound)> for Bound<NotNan<T>> {
//     type Error = ordered_float::FloatIsNan;
//     fn try_from(b: (T, Bound)) -> Result<Self, Self::Error> {
//         match b {
//             Bound::Inclusive(t) => NotNan::new(t).map(Self::Inclusive),
//             Bound::Exclusive(t) => NotNan::new(t).map(Self::Exclusive),
//         }
//     }
// }
// impl<T: ordered_float::FloatCore> TryFrom<(T, Inclusive)> for Bound<NotNan<T>> {
//     type Error = ordered_float::FloatIsNan;
//     fn try_from(b: (T, Inclusive)) -> Result<Self, Self::Error> {
//         Bound::from(b).try_into()
//     }
// }
// impl<T: ordered_float::FloatCore> TryFrom<(T, Exclusive)> for Bound<NotNan<T>> {
//     type Error = ordered_float::FloatIsNan;
//     fn try_from(b: (T, Exclusive)) -> Result<Self, Self::Error> {
//         Bound::from(b).try_into()
//     }
// }

impl<T: Ord + Clone> TryFrom<std::ops::Range<T>> for Interval<T, Inclusive, Exclusive> {
    type Error = crate::IntervalIsEmpty;
    fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
        Self::new((r.start, Inclusive), (r.end, Exclusive))
    }
}
impl<T: Ord + Clone> TryFrom<std::ops::RangeInclusive<T>> for Interval<T, Inclusive> {
    type Error = crate::IntervalIsEmpty;
    fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
        let (lower, upper) = r.into_inner();
        Self::new((lower, Inclusive), (upper, Inclusive))
    }
}
