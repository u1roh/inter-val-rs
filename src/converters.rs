use crate::{boundary::Boundary, Bound, Exclusive, Inclusive, Interval, Lower, Upper};
use ordered_float::{FloatCore, FloatIsNan, NotNan};

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

impl<T, B: Boundary> From<(T, B)> for Lower<T, B> {
    fn from((t, b): (T, B)) -> Self {
        Lower { val: t, bound: b }
    }
}
impl<T, B: Boundary> From<(T, B)> for Upper<T, B> {
    fn from((t, b): (T, B)) -> Self {
        Upper { val: t, bound: b }
    }
}

impl<T> From<T> for Lower<T, Inclusive> {
    fn from(t: T) -> Self {
        (t, Inclusive).into()
    }
}
impl<T> From<T> for Lower<T, Exclusive> {
    fn from(t: T) -> Self {
        (t, Exclusive).into()
    }
}
impl<T> From<T> for Upper<T, Inclusive> {
    fn from(t: T) -> Self {
        (t, Inclusive).into()
    }
}
impl<T> From<T> for Upper<T, Exclusive> {
    fn from(t: T) -> Self {
        (t, Exclusive).into()
    }
}

impl<T> From<Lower<T, Inclusive>> for Lower<T, Bound> {
    fn from(src: Lower<T, Inclusive>) -> Self {
        (src.val, Bound::Inclusive).into()
    }
}
impl<T> From<Lower<T, Exclusive>> for Lower<T, Bound> {
    fn from(src: Lower<T, Exclusive>) -> Self {
        (src.val, Bound::Exclusive).into()
    }
}
impl<T> From<Upper<T, Inclusive>> for Upper<T, Bound> {
    fn from(src: Upper<T, Inclusive>) -> Self {
        (src.val, Bound::Inclusive).into()
    }
}
impl<T> From<Upper<T, Exclusive>> for Upper<T, Bound> {
    fn from(src: Upper<T, Exclusive>) -> Self {
        (src.val, Bound::Exclusive).into()
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
