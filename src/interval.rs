use ordered_float::{FloatCore, NotNan};

use crate::bounding::{Left, Right};
use crate::traits::{BoundaryOf, Flip, IntoGeneral, Maximum, Minimum, Scalar};
use crate::{Bound, Exclusive, Inclusive, IntervalIsEmpty, LeftBounded, RightBounded};

/// Return type of `Interval::union()`.
pub struct IntervalUnion<T, L: Flip, R: Flip> {
    pub enclosure: Interval<T, L, R>,
    pub gap: Option<Interval<T, R::Flip, L::Flip>>,
}

fn is_valid_interval<T, L, R>(left: &LeftBounded<T, L>, right: &RightBounded<T, R>) -> bool
where
    T: Ord,
    L: BoundaryOf<Left>,
    R: BoundaryOf<Right>,
{
    left.contains(&right.val) && right.contains(&left.val)
}

/// Interval type.
#[derive(Debug, Clone, Copy, Eq)]
pub struct Interval<T, L = crate::Bounding, R = L> {
    left: LeftBounded<T, L>,
    right: RightBounded<T, R>,
}
impl<T: Eq, L: Eq, R: Eq> PartialEq for Interval<T, L, R> {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}
impl<T: Ord, L: BoundaryOf<Left>, R: BoundaryOf<Right>> Interval<T, L, R> {
    fn new_(left: LeftBounded<T, L>, right: RightBounded<T, R>) -> Result<Self, IntervalIsEmpty> {
        if is_valid_interval(&left, &right) {
            Ok(Self { left, right })
        } else {
            Err(IntervalIsEmpty)
        }
    }

    /// Create a new interval.
    /// ```
    /// # use std::any::{Any, TypeId};
    /// use intervals::{Interval, Bounding, Exclusive, Inclusive};
    ///
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::new(0.into(), 3.into()).unwrap();
    /// assert!(a.contains(&0));
    /// assert!(a.contains(&2));
    /// assert!(!a.contains(&3));
    ///
    /// let a = Interval::new(Exclusive.at(0), Inclusive.at(3)).unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Exclusive, Inclusive>>());
    ///
    /// let a = Interval::new(Bounding::Exclusive.at(0), Bounding::Exclusive.at(3)).unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Bounding, Bounding>>());
    /// ```
    pub fn new(left: Bound<T, L>, right: Bound<T, R>) -> Result<Self, IntervalIsEmpty> {
        Self::new_(left.into(), right.into())
    }

    pub fn try_new<T2>(left: Bound<T2, L>, right: Bound<T2, R>) -> Result<Self, crate::Error>
    where
        T: Scalar<T2>,
        crate::Error: From<T::Error>,
    {
        let left = Bound {
            val: T::scalar_try_from(left.val)?,
            bounding: left.bounding,
        };
        let right = Bound {
            val: T::scalar_try_from(right.val)?,
            bounding: right.bounding,
        };
        Self::new(left, right).map_err(Into::into)
    }

    pub fn left(&self) -> &LeftBounded<T, L> {
        &self.left
    }
    pub fn right(&self) -> &RightBounded<T, R> {
        &self.right
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Inclusive.at(7)).unwrap();
    /// let b = Exclusive.at(4).to(Inclusive.at(7)).unwrap();
    /// let c = Inclusive.at(1.23).float_to(Inclusive.at(4.56)).unwrap();
    /// assert_eq!(a.min(), 4);
    /// assert_eq!(b.min(), 5);
    /// assert_eq!(c.min(), 1.23);
    /// ```
    pub fn min(&self) -> T
    where
        LeftBounded<T, L>: Minimum<T>,
    {
        self.left.minimum()
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Inclusive.at(7)).unwrap();
    /// let b = Inclusive.at(4).to(Exclusive.at(7)).unwrap();
    /// let c = Inclusive.at(1.23).float_to(Inclusive.at(4.56)).unwrap();
    /// assert_eq!(a.max(), 7);
    /// assert_eq!(b.max(), 6);
    /// assert_eq!(c.max(), 4.56);
    /// ```
    pub fn max(&self) -> T
    where
        RightBounded<T, R>: Maximum<T>,
    {
        self.right.maximum()
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Exclusive.at(7)).unwrap();
    /// let b = Exclusive.at(1.23).float_to(Inclusive.at(4.56)).unwrap();
    /// assert!(a.contains(&4));
    /// assert!(!a.contains(&7));
    /// assert!(!b.contains(&1.23));
    /// assert!(b.contains(&4.56));
    /// ```
    pub fn contains<T2>(&self, t: &T2) -> bool
    where
        T: Scalar<T2>,
    {
        self.left.contains(t) && self.right.contains(t)
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.left.includes(&other.left) && self.right.includes(&other.right)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let left = std::cmp::max(&self.left, &other.left);
        let right = std::cmp::min(&self.right, &other.right);
        is_valid_interval(left, right)
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
        L::Flip: BoundaryOf<Right>,
        R::Flip: BoundaryOf<Left>,
    {
        Interval::new_(self.right.flip(), other.left.flip())
            .or(Interval::new_(other.right.flip(), self.left.flip()))
            .ok()
    }

    pub fn union(self, other: Self) -> IntervalUnion<T, L, R>
    where
        T: Clone,
        L::Flip: BoundaryOf<Right>,
        R::Flip: BoundaryOf<Left>,
    {
        IntervalUnion {
            gap: self.clone().gap(other.clone()),
            enclosure: self.enclosure(other),
        }
    }

    pub fn lower_bound(&self) -> RightBounded<T, L::Flip>
    where
        T: Clone,
    {
        self.left.clone().flip()
    }

    pub fn upper_bound(&self) -> LeftBounded<T, R::Flip>
    where
        T: Clone,
    {
        self.right.clone().flip()
    }

    pub fn enclosure_of<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.enclosure(item.into())))
    }
}
impl<T: FloatCore, L: BoundaryOf<Left>, R: BoundaryOf<Right>> Interval<NotNan<T>, L, R> {
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
