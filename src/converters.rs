use crate::{Bound, Exclusive, Inclusion, Inclusive, Interval};
use ordered_float::{FloatCore, NotNan};

pub(crate) trait IntoGeneral {
    type General;
    fn into_general(self) -> Self::General;
}

impl IntoGeneral for Inclusive {
    type General = Inclusion;
    fn into_general(self) -> Self::General {
        Inclusion::Inclusive
    }
}
impl IntoGeneral for Exclusive {
    type General = Inclusion;
    fn into_general(self) -> Self::General {
        Inclusion::Exclusive
    }
}

impl<T> From<T> for Bound<T, Inclusive> {
    fn from(t: T) -> Self {
        Self {
            val: t,
            inclusion: Inclusive,
        }
    }
}
impl<T> From<T> for Bound<T, Exclusive> {
    fn from(t: T) -> Self {
        Self {
            val: t,
            inclusion: Exclusive,
        }
    }
}

impl<T> From<Interval<T, Inclusive>> for Interval<T> {
    fn from(i: Interval<T, Inclusive>) -> Self {
        i.into_general()
    }
}
impl<T> From<Interval<T, Exclusive>> for Interval<T> {
    fn from(i: Interval<T, Exclusive>) -> Self {
        i.into_general()
    }
}
impl<T> From<Interval<T, Inclusive, Exclusive>> for Interval<T> {
    fn from(i: Interval<T, Inclusive, Exclusive>) -> Self {
        i.into_general()
    }
}
impl<T> From<Interval<T, Exclusive, Inclusive>> for Interval<T> {
    fn from(i: Interval<T, Exclusive, Inclusive>) -> Self {
        i.into_general()
    }
}

impl<T: Ord + Clone> From<T> for Interval<T, Inclusive> {
    fn from(t: T) -> Self {
        Self::new(t.clone().into(), t.into()).unwrap()
    }
}

impl<T: Ord + Clone> TryFrom<std::ops::Range<T>> for Interval<T, Inclusive, Exclusive> {
    type Error = crate::IntervalIsEmpty;
    fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
        Self::new(r.start.into(), r.end.into())
    }
}
impl<T: Ord + Clone> TryFrom<std::ops::RangeInclusive<T>> for Interval<T, Inclusive> {
    type Error = crate::IntervalIsEmpty;
    fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
        let (left, right) = r.into_inner();
        Self::new(left.into(), right.into())
    }
}

impl<T: FloatCore> TryFrom<std::ops::Range<T>> for Interval<NotNan<T>, Inclusive, Exclusive> {
    type Error = crate::Error;
    fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
        Self::not_nan(Inclusive.at(r.start), Exclusive.at(r.end))
    }
}
impl<T: FloatCore> TryFrom<std::ops::RangeInclusive<T>> for Interval<NotNan<T>, Inclusive> {
    type Error = crate::Error;
    fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
        let (left, right) = r.into_inner();
        Self::not_nan(Inclusive.at(left), Inclusive.at(right))
    }
}
