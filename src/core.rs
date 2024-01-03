use ordered_float::{FloatCore, NotNan};

use crate::boundary::Boundary;
use crate::{
    Bound, Exclusive, Inclusion, Inclusive, IntervalIsEmpty, LeftBounded, Maximum, Minimum,
    RightBounded,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T, L = crate::Inclusion, U = L> {
    lower: LeftBounded<T, L>,
    upper: RightBounded<T, U>,
}
impl<T: Ord, L: Boundary, U: Boundary> Interval<T, L, U> {
    fn new_(lower: LeftBounded<T, L>, upper: RightBounded<T, U>) -> Result<Self, IntervalIsEmpty> {
        (lower.contains(&upper.val) && upper.contains(&lower.val))
            .then_some(Self { lower, upper })
            .ok_or(IntervalIsEmpty)
    }
    pub fn new(lower: Bound<T, L>, upper: Bound<T, U>) -> Result<Self, IntervalIsEmpty> {
        Self::new_(lower.into(), upper.into())
    }
    pub fn lower(&self) -> &LeftBounded<T, L> {
        &self.lower
    }
    pub fn upper(&self) -> &RightBounded<T, U> {
        &self.upper
    }

    pub fn min_val(&self) -> T
    where
        LeftBounded<T, L>: Minimum<T>,
    {
        self.lower.minimum()
    }

    pub fn max_val(&self) -> T
    where
        RightBounded<T, U>: Maximum<T>,
    {
        self.upper.maximum()
    }

    pub fn contains(&self, t: &T) -> bool {
        self.lower.contains(t) && self.upper.contains(t)
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.lower.includes(&other.lower) && self.upper.includes(&other.upper)
    }

    pub fn intersection(self, other: Self) -> Option<Self> {
        Self::new_(
            self.lower.intersection(other.lower),
            self.upper.intersection(other.upper),
        )
        .ok()
    }

    pub fn union(self, other: Self) -> Self {
        Self {
            lower: self.lower.union(other.lower),
            upper: self.upper.union(other.upper),
        }
    }

    pub fn gap(self, other: Self) -> Option<Interval<T, U::Flip, L::Flip>> {
        Interval::new_(self.upper.flip(), other.lower.flip())
            .or(Interval::new_(other.upper.flip(), self.lower.flip()))
            .ok()
    }

    pub fn enclose<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.union(item.into())))
    }
}
impl<T: Ord + Clone, L: Boundary, U: Boundary> Interval<T, L, U> {
    #[allow(clippy::type_complexity)]
    pub fn union_strict(self, other: Self) -> (Self, Option<Interval<T, U::Flip, L::Flip>>) {
        let gap = self.clone().gap(other.clone());
        (self.union(other), gap)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.clone().intersection(other.clone()).is_some()
    }
}
impl<T: FloatCore, L: Boundary, U: Boundary> Interval<NotNan<T>, L, U> {
    pub fn not_nan(
        lower: impl Into<Bound<T, L>>,
        upper: impl Into<Bound<T, U>>,
    ) -> Result<Self, crate::Error> {
        let lower = lower.into().into_not_nan()?;
        let upper = upper.into().into_not_nan()?;
        Self::new(lower, upper).map_err(Into::into)
    }
    pub fn inf(&self) -> NotNan<T> {
        self.lower.inf()
    }
    pub fn sup(&self) -> NotNan<T> {
        self.upper.sup()
    }
    pub fn measure(&self) -> NotNan<T> {
        self.upper.val - self.lower.val
    }
    pub fn center(&self) -> NotNan<T> {
        NotNan::new((*self.lower.val + *self.upper.val) / (T::one() + T::one())).unwrap()
    }
    pub fn contains_f(&self, t: T) -> bool {
        NotNan::new(t).map(|t| self.contains(&t)).unwrap_or(false)
    }
    pub fn closure(self) -> Interval<NotNan<T>, Inclusive> {
        Interval {
            lower: self.lower.closure(),
            upper: self.upper.closure(),
        }
    }
    pub fn interior(self) -> Option<Interval<NotNan<T>, Exclusive>> {
        Interval::new_(self.lower.interior(), self.upper.interior()).ok()
    }
}
impl<T> Interval<T> {
    pub fn convert_from<L, U>(src: Interval<T, L, U>) -> Self
    where
        LeftBounded<T, L>: Into<LeftBounded<T, Inclusion>>,
        RightBounded<T, U>: Into<RightBounded<T, Inclusion>>,
    {
        Self {
            lower: src.lower.into(),
            upper: src.upper.into(),
        }
    }
}

// pub trait IntervalSet<T>: std::ops::Deref<Target = [Self::Interval]> {
//     type Interval: Interval<T>;
//     type Complement: IntervalSet<T>;
//     type Difference: IntervalSet<T>;
//     fn intersection(&self, other: &Self) -> Self;
//     fn union(&self, other: &Self) -> Self;
//     fn complement(&self) -> Self::Complement;
//     fn measure(&self) -> T;
//     fn overlaps(&self, other: &Self) -> bool;
// }
