use crate::{boundary::Boundary, Bound, Exclusive, Inclusion, Inclusive, Interval};
use ordered_float::{FloatCore, FloatIsNan, NotNan};

impl From<Inclusive> for Inclusion {
    fn from(_: Inclusive) -> Self {
        Self::Inclusive
    }
}
impl From<Exclusive> for Inclusion {
    fn from(_: Exclusive) -> Self {
        Self::Exclusive
    }
}

impl<T> From<Bound<T, Inclusive>> for Bound<T, Inclusion> {
    fn from(b: Bound<T, Inclusive>) -> Self {
        Self {
            val: b.val,
            inclusion: b.inclusion.into(),
        }
    }
}
impl<T> From<Bound<T, Exclusive>> for Bound<T, Inclusion> {
    fn from(b: Bound<T, Exclusive>) -> Self {
        Self {
            val: b.val,
            inclusion: b.inclusion.into(),
        }
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

impl<T, B: Boundary> From<(T, B)> for Bound<T, B> {
    fn from((t, b): (T, B)) -> Self {
        Self {
            val: t,
            inclusion: b,
        }
    }
}

impl<T> From<Interval<T, Inclusive>> for Interval<T> {
    fn from(i: Interval<T, Inclusive>) -> Self {
        Self::convert_from(i)
    }
}
impl<T> From<Interval<T, Exclusive>> for Interval<T> {
    fn from(i: Interval<T, Exclusive>) -> Self {
        Self::convert_from(i)
    }
}
impl<T> From<Interval<T, Inclusive, Exclusive>> for Interval<T> {
    fn from(i: Interval<T, Inclusive, Exclusive>) -> Self {
        Self::convert_from(i)
    }
}
impl<T> From<Interval<T, Exclusive, Inclusive>> for Interval<T> {
    fn from(i: Interval<T, Exclusive, Inclusive>) -> Self {
        Self::convert_from(i)
    }
}

impl<T: FloatCore, B: Boundary> crate::IntoNotNanBound<B> for (T, B) {
    type Float = T;
    fn into_not_nan_boundary(self) -> Result<(NotNan<T>, B), FloatIsNan> {
        let (t, b) = self;
        NotNan::new(t).map(|t| (t, b))
    }
}

macro_rules! impl_into_not_nan_bound {
    ($t:ty,$b:ident) => {
        impl crate::IntoNotNanBound<$b> for $t {
            type Float = $t;
            fn into_not_nan_boundary(self) -> Result<(NotNan<$t>, $b), FloatIsNan> {
                NotNan::new(self).map(|t| (t, $b))
            }
        }
    };
}
impl_into_not_nan_bound!(f32, Inclusive);
impl_into_not_nan_bound!(f32, Exclusive);
impl_into_not_nan_bound!(f64, Inclusive);
impl_into_not_nan_bound!(f64, Exclusive);

impl<T: Ord + Clone> From<T> for Interval<T, Inclusive> {
    fn from(t: T) -> Self {
        Self::new(t.clone(), t).unwrap()
    }
}

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

impl<T: FloatCore> TryFrom<std::ops::Range<T>> for Interval<NotNan<T>, Inclusive, Exclusive> {
    type Error = crate::Error;
    fn try_from(r: std::ops::Range<T>) -> Result<Self, Self::Error> {
        Self::not_nan((r.start, Inclusive), (r.end, Exclusive))
    }
}
impl<T: FloatCore> TryFrom<std::ops::RangeInclusive<T>> for Interval<NotNan<T>, Inclusive> {
    type Error = crate::Error;
    fn try_from(r: std::ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
        let (lower, upper) = r.into_inner();
        Self::not_nan((lower, Inclusive), (upper, Inclusive))
    }
}
