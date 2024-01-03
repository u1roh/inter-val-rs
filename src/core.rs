use ordered_float::{FloatCore, NotNan};

use crate::boundary::Boundary;
use crate::{Bound, Exclusive, Inclusive, IntervalIsEmpty, IntoNotNanBound, Lower, Upper};

pub trait MinVal<T> {
    fn min_val(&self) -> T;
}
pub trait MaxVal<T> {
    fn max_val(&self) -> T;
}

impl<T: Ord, B: Boundary> Lower<T, B> {
    pub fn includes(&self, other: &Self) -> bool {
        self.val <= other.val
    }
    pub fn contains(&self, t: &T) -> bool {
        self.bound.less(&self.val, t)
    }
    pub fn intersection(self, other: Self) -> Self {
        self.max(other)
    }
    pub fn union(self, other: Self) -> Self {
        self.min(other)
    }
    pub fn flip(self) -> Upper<T, B::Flip> {
        Upper {
            val: self.val,
            bound: self.bound.flip(),
        }
    }
}
impl<T: FloatCore, B: Boundary> Lower<NotNan<T>, B> {
    pub fn closure(self) -> Lower<NotNan<T>, Inclusive> {
        Lower {
            val: self.val,
            bound: Inclusive,
        }
    }
    pub fn interior(self) -> Lower<NotNan<T>, Exclusive> {
        Lower {
            val: self.val,
            bound: Exclusive,
        }
    }
    pub fn inf(&self) -> NotNan<T> {
        self.val
    }
}
impl<T: Clone> MinVal<T> for Lower<T, Inclusive> {
    fn min_val(&self) -> T {
        self.val.clone()
    }
}
impl<T: num::Integer + Clone> MinVal<T> for Lower<T, Exclusive> {
    fn min_val(&self) -> T {
        self.val.clone() + T::one()
    }
}
impl<T: num::Integer + Clone> MinVal<T> for Lower<T, Bound> {
    fn min_val(&self) -> T {
        match self.bound {
            Bound::Inclusive => self.val.clone(),
            Bound::Exclusive => self.val.clone() + T::one(),
        }
    }
}

impl<T: Ord, B: Boundary> Upper<T, B> {
    pub fn includes(&self, other: &Self) -> bool {
        other.val <= self.val
    }
    pub fn contains(&self, t: &T) -> bool {
        self.bound.less(t, &self.val)
    }
    pub fn intersection(self, other: Self) -> Self {
        self.min(other)
    }
    pub fn union(self, other: Self) -> Self {
        self.max(other)
    }
    pub fn flip(self) -> Lower<T, B::Flip> {
        Lower {
            val: self.val,
            bound: self.bound.flip(),
        }
    }
}
impl<T: FloatCore, B: Boundary> Upper<NotNan<T>, B> {
    pub fn closure(self) -> Upper<NotNan<T>, Inclusive> {
        Upper {
            val: self.val,
            bound: Inclusive,
        }
    }
    pub fn interior(self) -> Upper<NotNan<T>, Exclusive> {
        Upper {
            val: self.val,
            bound: Exclusive,
        }
    }
    pub fn sup(&self) -> NotNan<T> {
        self.val
    }
}
impl<T: Clone> MaxVal<T> for Upper<T, Inclusive> {
    fn max_val(&self) -> T {
        self.val.clone()
    }
}
impl<T: num::Integer + Clone> MaxVal<T> for Upper<T, Exclusive> {
    fn max_val(&self) -> T {
        self.val.clone() - T::one()
    }
}
impl<T: num::Integer + Clone> MaxVal<T> for Upper<T, Bound> {
    fn max_val(&self) -> T {
        match self.bound {
            Bound::Inclusive => self.val.clone(),
            Bound::Exclusive => self.val.clone() - T::one(),
        }
    }
}

pub type UnionSubtrahend<T, L, U> = Interval<T, <U as Boundary>::Flip, <L as Boundary>::Flip>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T, L = crate::Bound, U = L> {
    lower: Lower<T, L>,
    upper: Upper<T, U>,
}
impl<T: Ord, L: Boundary, U: Boundary> Interval<T, L, U> {
    pub fn new(
        lower: impl Into<Lower<T, L>>,
        upper: impl Into<Upper<T, U>>,
    ) -> Result<Self, IntervalIsEmpty> {
        let lower = lower.into();
        let upper = upper.into();
        (lower.contains(&upper.val) && upper.contains(&lower.val))
            .then_some(Self { lower, upper })
            .ok_or(IntervalIsEmpty)
    }
    pub fn lower(&self) -> &Lower<T, L> {
        &self.lower
    }
    pub fn upper(&self) -> &Upper<T, U> {
        &self.upper
    }

    pub fn min_val(&self) -> T
    where
        Lower<T, L>: MinVal<T>,
    {
        self.lower.min_val()
    }

    pub fn max_val(&self) -> T
    where
        Upper<T, U>: MaxVal<T>,
    {
        self.upper.max_val()
    }

    pub fn contains(&self, t: &T) -> bool {
        self.lower.contains(t) && self.upper.contains(t)
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.lower.includes(&other.lower) && self.upper.includes(&other.upper)
    }

    pub fn intersection(self, other: Self) -> Option<Self> {
        Self::new(
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
        Interval::new(self.upper.flip(), other.lower.flip())
            .or(Interval::new(other.upper.flip(), self.lower.flip()))
            .ok()
    }

    pub fn bound<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
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
        lower: impl IntoNotNanBound<L, Float = T>,
        upper: impl IntoNotNanBound<U, Float = T>,
    ) -> Result<Self, crate::Error> {
        let lower = lower.into_not_nan_boundary()?;
        let upper = upper.into_not_nan_boundary()?;
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
    pub fn closure(self) -> Interval<NotNan<T>, Inclusive> {
        Interval {
            lower: self.lower.closure(),
            upper: self.upper.closure(),
        }
    }
    pub fn interior(self) -> Option<Interval<NotNan<T>, Exclusive>> {
        Interval::new(self.lower.interior(), self.upper.interior()).ok()
    }
}
impl<T> Interval<T> {
    pub fn convert_from<L, U>(src: Interval<T, L, U>) -> Self
    where
        Lower<T, L>: Into<Lower<T, Bound>>,
        Upper<T, U>: Into<Upper<T, Bound>>,
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
