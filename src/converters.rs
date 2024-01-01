use crate::{Bound, Exclusive, Inclusive, Interval};
use ordered_float::NotNan;

impl<T> From<T> for Inclusive<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}
impl<T> From<T> for Exclusive<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> From<Inclusive<T>> for Bound<T> {
    fn from(b: Inclusive<T>) -> Self {
        Self::Inclusive(b.0)
    }
}
impl<T> From<Exclusive<T>> for Bound<T> {
    fn from(b: Exclusive<T>) -> Self {
        Self::Exclusive(b.0)
    }
}

impl<T: ordered_float::FloatCore> TryFrom<Inclusive<T>> for Inclusive<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Inclusive<T>) -> Result<Self, Self::Error> {
        NotNan::new(b.0).map(Self)
    }
}
impl<T: ordered_float::FloatCore> TryFrom<Exclusive<T>> for Exclusive<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Exclusive<T>) -> Result<Self, Self::Error> {
        NotNan::new(b.0).map(Self)
    }
}
impl<T: ordered_float::FloatCore> TryFrom<Bound<T>> for Bound<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Bound<T>) -> Result<Self, Self::Error> {
        match b {
            Bound::Inclusive(t) => NotNan::new(t).map(Self::Inclusive),
            Bound::Exclusive(t) => NotNan::new(t).map(Self::Exclusive),
        }
    }
}
impl<T: ordered_float::FloatCore> TryFrom<Inclusive<T>> for Bound<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Inclusive<T>) -> Result<Self, Self::Error> {
        Bound::from(b).try_into()
    }
}
impl<T: ordered_float::FloatCore> TryFrom<Exclusive<T>> for Bound<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Exclusive<T>) -> Result<Self, Self::Error> {
        Bound::from(b).try_into()
    }
}

impl<L: crate::boundary::Boundary, U: crate::boundary::Boundary<Val = L::Val>> TryFrom<(L, U)>
    for Interval<L, U>
{
    type Error = crate::IntervalIsEmpty;
    fn try_from((l, u): (L, U)) -> Result<Self, Self::Error> {
        Self::new(l, u)
    }
}

impl<T: Ord> TryFrom<std::ops::Range<T>> for Interval<Inclusive<T>, Exclusive<T>> {
    type Error = crate::IntervalIsEmpty;
    fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
        Self::new(Inclusive(r.start), Exclusive(r.end))
    }
}
impl<T: Ord> TryFrom<std::ops::RangeInclusive<T>> for Interval<Inclusive<T>> {
    type Error = crate::IntervalIsEmpty;
    fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
        let (lower, upper) = r.into_inner();
        Self::new(Inclusive(lower), Inclusive(upper))
    }
}
