use ordered_float::{FloatCore, NotNan};

use crate::converters::IntoGeneral;
use crate::half::{LeftInclusion, RightInclusion};
use crate::traits::{Boundary, Maximum, Minimum};
use crate::{Bound, Exclusive, Inclusive, IntervalIsEmpty, LeftBounded, RightBounded};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T, L = crate::Inclusion, R = L> {
    left: LeftBounded<T, L>,
    right: RightBounded<T, R>,
}
impl<T: Ord, L: Boundary, R: Boundary> Interval<T, L, R>
where
    LeftInclusion<L>: Ord,
    RightInclusion<R>: Ord,
{
    fn new_(left: LeftBounded<T, L>, right: RightBounded<T, R>) -> Result<Self, IntervalIsEmpty> {
        (left.contains(&right.val) && right.contains(&left.val))
            .then_some(Self { left, right })
            .ok_or(IntervalIsEmpty)
    }
    pub fn new(left: Bound<T, L>, right: Bound<T, R>) -> Result<Self, IntervalIsEmpty> {
        Self::new_(left.into(), right.into())
    }
    pub fn left(&self) -> &LeftBounded<T, L> {
        &self.left
    }
    pub fn right(&self) -> &RightBounded<T, R> {
        &self.right
    }

    pub fn min(&self) -> T
    where
        LeftBounded<T, L>: Minimum<T>,
    {
        self.left.minimum()
    }

    pub fn max(&self) -> T
    where
        RightBounded<T, R>: Maximum<T>,
    {
        self.right.maximum()
    }

    pub fn contains(&self, t: &T) -> bool {
        self.left.contains(t) && self.right.contains(t)
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.left.includes(&other.left) && self.right.includes(&other.right)
    }

    pub fn intersection(self, other: Self) -> Option<Self> {
        Self::new_(
            self.left.intersection(other.left),
            self.right.intersection(other.right),
        )
        .ok()
    }

    pub fn enclosure(self, other: Self) -> Self {
        Self {
            left: self.left.union(other.left),
            right: self.right.union(other.right),
        }
    }

    pub fn gap(self, other: Self) -> Option<Interval<T, R::Flip, L::Flip>>
    where
        LeftInclusion<R::Flip>: Ord,
        RightInclusion<L::Flip>: Ord,
    {
        Interval::new_(self.right.flip(), other.left.flip())
            .or(Interval::new_(other.right.flip(), self.left.flip()))
            .ok()
    }

    pub fn new_enclosure<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.enclosure(item.into())))
    }
}
impl<T: Ord + Clone, L: Boundary, R: Boundary> Interval<T, L, R>
where
    LeftInclusion<L>: Ord,
    RightInclusion<R>: Ord,
{
    #[allow(clippy::type_complexity)]
    pub fn union(self, other: Self) -> (Self, Option<Interval<T, R::Flip, L::Flip>>)
    where
        LeftInclusion<R::Flip>: Ord,
        RightInclusion<L::Flip>: Ord,
    {
        let gap = self.clone().gap(other.clone());
        (self.enclosure(other), gap)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.clone().intersection(other.clone()).is_some()
    }
}
impl<T: FloatCore, L: Boundary, R: Boundary> Interval<NotNan<T>, L, R>
where
    LeftInclusion<L>: Ord,
    RightInclusion<R>: Ord,
{
    pub fn not_nan(
        left: impl Into<Bound<T, L>>,
        right: impl Into<Bound<T, R>>,
    ) -> Result<Self, crate::Error> {
        let left = left.into().into_not_nan()?;
        let right = right.into().into_not_nan()?;
        Self::new(left, right).map_err(Into::into)
    }
    pub fn inf(&self) -> NotNan<T> {
        self.left.inf()
    }
    pub fn sup(&self) -> NotNan<T> {
        self.right.sup()
    }
    pub fn measure(&self) -> NotNan<T> {
        self.right.val - self.left.val
    }
    pub fn center(&self) -> NotNan<T> {
        NotNan::new((*self.left.val + *self.right.val) / (T::one() + T::one())).unwrap()
    }
    pub fn contains_f(&self, t: T) -> bool {
        NotNan::new(t).map(|t| self.contains(&t)).unwrap_or(false)
    }
    pub fn closure(self) -> Interval<NotNan<T>, Inclusive> {
        Interval {
            left: self.left.closure(),
            right: self.right.closure(),
        }
    }
    pub fn interior(self) -> Option<Interval<NotNan<T>, Exclusive>> {
        Interval::<_, Exclusive>::new_(self.left.interior(), self.right.interior()).ok()
    }
}

impl<T, L: IntoGeneral, R: IntoGeneral> IntoGeneral for Interval<T, L, R> {
    type General = Interval<T, L::General, R::General>;
    fn into_general(self) -> Self::General {
        Interval {
            left: self.left.into_general(),
            right: self.right.into_general(),
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
