use crate::{boundary::Boundary, Bound, Exclusive, Inclusive, Interval, Lower, Upper};

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

impl<T> From<T> for Lower<T, Inclusive> {
    fn from(t: T) -> Self {
        Lower {
            val: t,
            bound: Inclusive,
        }
    }
}
impl<T> From<T> for Lower<T, Exclusive> {
    fn from(t: T) -> Self {
        Lower {
            val: t,
            bound: Exclusive,
        }
    }
}
// impl<T, B: Boundary<T>, B2: Into<B>> From<(T, B2)> for Lower<T, B> {
//     fn from((t, b): (T, B2)) -> Self {
//         Lower {
//             val: t,
//             bound: b.into(),
//         }
//     }
// }
impl<T, B> From<(T, B)> for Lower<T, B> {
    fn from((t, b): (T, B)) -> Self {
        Lower { val: t, bound: b }
    }
}

impl<T> From<T> for Upper<T, Inclusive> {
    fn from(t: T) -> Self {
        Upper {
            val: t,
            bound: Inclusive,
        }
    }
}
impl<T> From<T> for Upper<T, Exclusive> {
    fn from(t: T) -> Self {
        Upper {
            val: t,
            bound: Exclusive,
        }
    }
}
// impl<T, B: Boundary<T>, B2: Into<B>> From<(T, B2)> for Upper<T, B> {
//     fn from((t, b): (T, B2)) -> Self {
//         Upper {
//             val: t,
//             bound: b.into(),
//         }
//     }
// }
impl<T, B> From<(T, B)> for Upper<T, B> {
    fn from((t, b): (T, B)) -> Self {
        Upper { val: t, bound: b }
    }
}

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
        Self::new(r.start, r.end)
    }
}
impl<T: Ord + Clone> TryFrom<std::ops::RangeInclusive<T>> for Interval<T, Inclusive> {
    type Error = crate::IntervalIsEmpty;
    fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
        let (lower, upper) = r.into_inner();
        Self::new(lower, upper)
    }
}
