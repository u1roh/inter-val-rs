use crate::bound_type::{Left, Right};
use crate::traits::{BoundaryOf, Flip, IntoGeneral, Maximum, Minimum};
use crate::{Bound, Exclusive, Inclusive, LeftBounded, RightBounded};

/// Return type of `Interval::union()`.
pub struct IntervalUnion<T, L: Flip, R: Flip> {
    pub span: Interval<T, L, R>,
    pub gap: Option<Interval<T, R::Flip, L::Flip>>,
}
impl<T, L: Flip, R: Flip> IntoIterator for IntervalUnion<T, L, R> {
    type Item = Interval<T, L, R>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        if let Some(gap) = self.gap {
            let first = Interval {
                left: self.span.left,
                right: gap.left.flip(),
            };
            let second = Interval {
                left: gap.right.flip(),
                right: self.span.right,
            };
            vec![first, second].into_iter()
        } else {
            vec![self.span].into_iter()
        }
    }
}

fn is_valid_interval<T, L, R>(left: &LeftBounded<T, L>, right: &RightBounded<T, R>) -> bool
where
    T: PartialOrd,
    L: BoundaryOf<Left>,
    R: BoundaryOf<Right>,
{
    left.contains(&right.limit) && right.contains(&left.limit)
}

/// Interval like *[a, b]*, *(a, b)*, *[a, b)*, and *(a, b]* for any `PartialOrd` type.
///
/// * `T`: Scalar type. `T` should implements `PartialOrd`. `NaN` safety is not guaranteed when `T` is floating point type.
/// * `L`: Left boundary type. Specify one of [`Inclusive`], [`Exclusive`], or [`BoundType`](crate::BoundType).
/// * `R`: Right boundary type. Specify one of [`Inclusive`] [`Exclusive`], or [`BoundType`](crate::BoundType).
/// * `Interval<T>` (= `Interval<T, Inclusive, Inclusive>`) represents a closed interval, i.e., *[a, b]*.
/// * `Interval<T, Exclusive>` (= `Interval<T, Exclusive, Exclusive>`) represents a open interval, i.e., *(a, b)*.
/// * `Interval<T, Inclusive, Exclusive>` represents a right half-open interval, i.e., *[a, b)*.
/// * `Interval<T, Exclusive, Inclusive>` represents a left half-open interval, i.e., *(a, b]*.
/// * `Interval<T, BoundType>` represents any of the above.
///
/// This type is considered as an interval on ℝ (real number line) even if an integer type is specified for `T`.
///
/// ```
/// use kd_interval::{Interval, Exclusive, Inclusive, BoundType};
/// assert_eq!(std::mem::size_of::<Interval<i32, Inclusive>>(), std::mem::size_of::<i32>() * 2);
/// assert_eq!(std::mem::size_of::<Interval<f64, Exclusive>>(), std::mem::size_of::<f64>() * 2);
/// assert!(std::mem::size_of::<Interval<i32, BoundType>>() > (std::mem::size_of::<i32>() + std::mem::size_of::<BoundType>()) * 2);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Interval<T, L = Inclusive, R = L> {
    pub(crate) left: LeftBounded<T, L>,
    pub(crate) right: RightBounded<T, R>,
}
impl<T: PartialEq, L: Eq, R: Eq> PartialEq for Interval<T, L, R> {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}
impl<T: Eq, L: Eq, R: Eq> Eq for Interval<T, L, R> {}

impl<T, L, R> Interval<T, L, R> {
    pub fn left(&self) -> &LeftBounded<T, L> {
        &self.left
    }
    pub fn right(&self) -> &RightBounded<T, R> {
        &self.right
    }
}
impl<T: PartialOrd, L: BoundaryOf<Left>, R: BoundaryOf<Right>> Interval<T, L, R> {
    fn new_(left: LeftBounded<T, L>, right: RightBounded<T, R>) -> Option<Self> {
        is_valid_interval(&left, &right).then_some(Self { left, right })
    }

    /// Try to create a new interval. Return `None` if the interval is empty.
    /// ```
    /// use std::any::{Any, TypeId};
    /// use kd_interval::{Interval, BoundType, Exclusive, Inclusive};
    ///
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::try_new(0.into(), 3.into()).unwrap();
    /// assert!(a.contains(&0));
    /// assert!(a.contains(&2));
    /// assert!(!a.contains(&3));
    ///
    /// let a = Interval::try_new(Exclusive.at(0), Inclusive.at(3)).unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Exclusive, Inclusive>>());
    ///
    /// let a = Interval::try_new(BoundType::Exclusive.at(0), BoundType::Exclusive.at(3)).unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, BoundType, BoundType>>());
    ///
    /// assert!(Interval::try_new(Inclusive.at(3), Exclusive.at(0)).is_none()); // [3, 0) is empty.
    /// assert!(Interval::try_new(Inclusive.at(3), Exclusive.at(3)).is_none()); // [3, 3) is empty.
    /// assert!(Interval::try_new(Inclusive.at(3), Inclusive.at(3)).is_some()); // [3, 3] is not empty.
    /// assert!(Interval::try_new(Exclusive.at(0), Exclusive.at(1)).is_some()); // (0, 1) is not empty.
    /// ```
    pub fn try_new(left: Bound<T, L>, right: Bound<T, R>) -> Option<Self> {
        Self::new_(left.into(), right.into())
    }

    /// Create a new interval. Panics if the interval is empty.
    /// ```
    /// use std::any::{Any, TypeId};
    /// use kd_interval::{Interval, BoundType, Exclusive, Inclusive};
    ///
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::new(0.into(), 3.into());
    /// assert!(a.contains(&0));
    /// assert!(a.contains(&2));
    /// assert!(!a.contains(&3));
    ///
    /// let a = Interval::new(Exclusive.at(0), Inclusive.at(3));
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Exclusive, Inclusive>>());
    ///
    /// let a = Interval::new(BoundType::Exclusive.at(0), BoundType::Exclusive.at(3));
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, BoundType, BoundType>>());
    /// ```
    ///
    /// # Panics
    /// ```should_panic
    /// # use kd_interval::{Interval, Exclusive, Inclusive};
    /// Interval::new(Inclusive.at(3), Exclusive.at(0)); // [3, 0) is empty.
    /// ```
    /// ```should_panic
    /// # use kd_interval::{Interval, Exclusive, Inclusive};
    /// Interval::new(Inclusive.at(3), Exclusive.at(3)); // [3, 3) is empty.
    /// ```
    pub fn new(left: Bound<T, L>, right: Bound<T, R>) -> Self {
        Self::try_new(left, right).expect("Invalid interval: left must be less than right.")
    }

    /// ```
    /// use kd_interval::{Interval, Exclusive, Inclusive};
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::try_between(-2, 5).unwrap();
    /// assert_eq!(a, Inclusive.at(-2).to(Exclusive.at(5)));
    ///
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::try_between(3, -1).unwrap();
    /// assert_eq!(a, Inclusive.at(-1).to(Exclusive.at(3))); // Swaps left and right.
    ///
    /// assert!(Interval::<i32, Inclusive, Exclusive>::try_between(1, 1).is_none()); // [1, 1) is empty.
    /// assert!(Interval::<i32, Inclusive, Inclusive>::try_between(1, 1).is_some()); // [1, 1] is not empty.
    /// ```
    pub fn try_between(a: T, b: T) -> Option<Self>
    where
        T: Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        if a < b {
            Self::try_new(a.into(), b.into())
        } else {
            Self::try_new(b.into(), a.into())
        }
    }

    /// ```
    /// use kd_interval::{Interval, Exclusive, Inclusive};
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::between(-2, 5);
    /// assert_eq!(a, Inclusive.at(-2).to(Exclusive.at(5)));
    ///
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::between(3, -1);
    /// assert_eq!(a, Inclusive.at(-1).to(Exclusive.at(3))); // Swaps left and right.
    ///
    /// // Closed interval (bounded by `Inclusive`) never panics.
    /// Interval::<i32, Inclusive, Inclusive>::between(1, 1); // Doesn't panic since [1, 1] is not empty.
    /// ```
    /// ```should_panic
    /// # use kd_interval::{Interval, Exclusive, Inclusive};
    /// Interval::<i32, Inclusive, Exclusive>::between(1, 1); // Panics since [1, 1) is empty.
    /// ```
    pub fn between(a: T, b: T) -> Self
    where
        T: Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        Self::try_between(a, b).unwrap()
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Inclusive.at(7));
    /// let b = Exclusive.at(4).to(Inclusive.at(7));
    /// let c = Inclusive.at(1.23).to(Inclusive.at(4.56));
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
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Inclusive.at(7));
    /// let b = Inclusive.at(4).to(Exclusive.at(7));
    /// let c = Inclusive.at(1.23).to(Inclusive.at(4.56));
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
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Exclusive.at(7));
    /// let b = Exclusive.at(1.23).to(Inclusive.at(4.56));
    /// assert!(a.contains(&4));
    /// assert!(!a.contains(&7));
    /// assert!(!b.contains(&1.23));
    /// assert!(b.contains(&1.230000000001));
    /// assert!(b.contains(&4.56));
    /// ```
    pub fn contains(&self, t: &T) -> bool {
        self.left.contains(t) && self.right.contains(t)
    }

    /// ```
    /// use kd_interval::{Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Exclusive.at(7));
    /// assert_eq!(a.dilate(2), Inclusive.at(2).to(Exclusive.at(9)));
    /// assert_eq!(a.dilate(-1), Inclusive.at(5).to(Exclusive.at(6)));
    /// assert!(std::panic::catch_unwind(|| a.dilate(-2)).is_err());
    /// ```
    pub fn dilate(self, delta: T) -> Self
    where
        T: Clone + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
    {
        Self::new_(self.left.dilate(delta.clone()), self.right.dilate(delta)).unwrap()
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));
    /// let b = Inclusive.at(0).to(Exclusive.at(4));
    /// let c = Inclusive.at(1).to(Exclusive.at(4));
    /// assert!(a.includes(&a));
    /// assert!(!a.includes(&b) && b.includes(&a));
    /// assert!(!a.includes(&c) && !c.includes(&a));
    /// ```
    pub fn includes(&self, other: &Self) -> bool {
        self.left.includes(&other.left) && self.right.includes(&other.right)
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));
    /// let b = Inclusive.at(1).to(Exclusive.at(4));
    /// let c = Inclusive.at(3).to(Exclusive.at(5));
    /// assert!(a.overlaps(&a));
    /// assert!(a.overlaps(&b) && b.overlaps(&a));
    /// assert!(!a.overlaps(&c) && !c.overlaps(&a));
    /// ```
    pub fn overlaps(&self, other: &Self) -> bool {
        let left = crate::half::partial_max(&self.left, &other.left);
        let right = crate::half::partial_min(&self.right, &other.right);
        is_valid_interval(left, right)
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));
    /// let b = Inclusive.at(1).to(Exclusive.at(4));
    /// let c = Inclusive.at(3).to(Exclusive.at(5));
    /// assert_eq!(a.intersection(&a), Some(a));
    /// assert_eq!(a.intersection(&b), Some(Inclusive.at(1).to(Exclusive.at(3))));
    /// assert_eq!(a.intersection(&c), None);
    /// ```
    pub fn intersection(&self, other: &Self) -> Option<Self>
    where
        T: Clone,
    {
        Self::new_(
            self.left.intersection(&other.left).clone(),
            self.right.intersection(&other.right).clone(),
        )
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));
    /// let b = Inclusive.at(5).to(Exclusive.at(8));
    /// assert_eq!(a.span(&b), Inclusive.at(0).to(Exclusive.at(8)));
    /// ```
    pub fn span(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        Self {
            left: self.left.union(&other.left).clone(),
            right: self.right.union(&other.right).clone(),
        }
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));
    /// assert_eq!(a.hull(-2), Inclusive.at(-2).to(Exclusive.at(3)));
    /// assert_eq!(a.hull(5), Inclusive.at(0).to(Exclusive.at(5)));
    /// ```
    pub fn hull(self, t: T) -> Self
    where
        T: Clone,
    {
        Self {
            left: self.left.hull(t.clone()),
            right: self.right.hull(t),
        }
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));
    /// let b = Inclusive.at(5).to(Exclusive.at(8));
    /// assert_eq!(a.gap(&b).unwrap(), Inclusive.at(3).to(Exclusive.at(5)));
    /// ```
    pub fn gap(&self, other: &Self) -> Option<Interval<T, R::Flip, L::Flip>>
    where
        T: Clone,
        L::Flip: BoundaryOf<Right>,
        R::Flip: BoundaryOf<Left>,
    {
        Interval::new_(self.right.clone().flip(), other.left.clone().flip())
            .or_else(|| Interval::new_(other.right.clone().flip(), self.left.clone().flip()))
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));
    /// let b = Inclusive.at(5).to(Exclusive.at(8));
    /// let union = a.union(&b);
    /// assert_eq!(union.span, a.span(&b));
    /// assert_eq!(union.gap, a.gap(&b));
    /// let union_ints: Vec<Interval<_, _, _>> = union.into_iter().collect();
    /// assert_eq!(union_ints.len(), 2);
    /// assert_eq!(union_ints[0], a);
    /// assert_eq!(union_ints[1], b);
    /// ```
    pub fn union(&self, other: &Self) -> IntervalUnion<T, L, R>
    where
        T: Clone,
        L::Flip: BoundaryOf<Right>,
        R::Flip: BoundaryOf<Left>,
    {
        IntervalUnion {
            gap: self.gap(other),
            span: self.span(other),
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

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive, Nullable};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));  // [0, 3)
    /// let b = Inclusive.at(1).to(Exclusive.at(5));  // [1, 5)
    /// let c = Inclusive.at(8).to(Exclusive.at(10)); // [8, 10)
    /// let span = Interval::span_many(vec![a, b, c]).unwrap(); // [0, 10)
    /// assert_eq!(span.left().limit, 0);
    /// assert_eq!(span.right().limit, 10);
    ///
    /// // Sum for Nullable<Interval> can be used as well.
    /// let sum: Nullable<Interval<_, _, _>> = vec![a, b, c].into_iter().sum();
    /// assert_eq!(sum.unwrap(), span);
    /// ```
    pub fn span_many<A: std::borrow::Borrow<Self>>(
        items: impl IntoIterator<Item = A>,
    ) -> Option<Self>
    where
        T: Clone,
    {
        let mut items = items.into_iter();
        let first = items.next()?.borrow().clone();
        Some(items.fold(first, |acc, item| acc.span(item.borrow())))
    }

    /// ```
    /// use kd_interval::{Interval, Nullable};
    /// let hull = Interval::<_>::hull_many(vec![3, 9, 2, 5]).unwrap(); // [2, 9]
    /// assert_eq!(hull.min(), 2);
    /// assert_eq!(hull.max(), 9);
    ///
    /// let hull = Interval::<_>::hull_many(vec![3.1, 9.2, 2.3, 5.4]).unwrap(); // [2.3, 9.2]
    /// assert_eq!(hull.inf(), 2.3);
    /// assert_eq!(hull.sup(), 9.2);
    ///
    /// // Sum for Nullable<Interval> can be used as well.
    /// let a: Nullable<Interval<i32>> = vec![1, 6, 2, 8, 3].into_iter().sum();
    /// assert_eq!(a.unwrap(), Interval::between(1, 8));
    /// ```
    pub fn hull_many(items: impl IntoIterator<Item = T>) -> Option<Self>
    where
        T: Clone + Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        let mut items = items.into_iter();
        let mut left = items.next()?;
        let mut right = left.clone();
        for x in items {
            if x < left {
                left = x;
            } else if right < x {
                right = x;
            }
        }
        Self::try_new(left.into(), right.into())
    }
}

impl<T: num::Float, L: BoundaryOf<Left>, R: BoundaryOf<Right>> Interval<T, L, R> {
    /// ```
    /// use kd_interval::{Interval, Exclusive, Inclusive};
    /// let a = Interval::new(Inclusive.at(-1.0), Inclusive.at(1.0));
    /// assert_eq!(a.inf(), -1.0);
    /// assert!(a.contains(&-1.0));
    ///
    /// let b = Interval::new(Exclusive.at(-1.0), Inclusive.at(1.0));
    /// assert_eq!(b.inf(), -1.0);
    /// assert!(!b.contains(&-1.0));
    /// ```
    pub fn inf(&self) -> T {
        self.left.inf()
    }

    /// ```
    /// use kd_interval::{Interval, Exclusive, Inclusive};
    /// let a = Interval::new(Inclusive.at(-1.0), Inclusive.at(1.0));
    /// assert_eq!(a.sup(), 1.0);
    /// assert!(a.contains(&1.0));
    ///
    /// let b = Interval::new(Inclusive.at(-1.0), Exclusive.at(1.0));
    /// assert_eq!(b.sup(), 1.0);
    /// assert!(!b.contains(&1.0));
    /// ```
    pub fn sup(&self) -> T {
        self.right.sup()
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive};
    /// let a = Inclusive.at(2.1).to(Inclusive.at(5.3));
    /// assert_eq!(a.measure(), 5.3 - 2.1);
    ///
    /// let a = Inclusive.at(std::f64::INFINITY).to(Inclusive.at(std::f64::INFINITY));
    /// assert!(a.measure().is_nan());
    /// ```
    pub fn measure(&self) -> T {
        self.right.limit - self.left.limit
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive};
    /// let a = Inclusive.at(2.1).to(Inclusive.at(5.3));
    /// assert_eq!(a.center(), (2.1 + 5.3) / 2.0);
    ///
    /// let a = Inclusive.at(std::f64::NEG_INFINITY).to(Inclusive.at(std::f64::INFINITY));
    /// assert!(a.center().is_nan());
    /// ```
    pub fn center(&self) -> T {
        (self.left.limit + self.right.limit) / (T::one() + T::one())
    }

    pub fn closure(self) -> Interval<T, Inclusive> {
        Interval {
            left: self.left.closure(),
            right: self.right.closure(),
        }
    }
    pub fn interior(self) -> Option<Interval<T, Exclusive>> {
        Interval::<_, Exclusive>::new_(self.left.interior(), self.right.interior())
    }

    /// IoU - Intersection over Union.
    /// ```
    /// use kd_interval::{Interval, Inclusive};
    /// let a = Inclusive.at(0.0).to(Inclusive.at(1.0));
    /// let b = Inclusive.at(0.0).to(Inclusive.at(2.0));
    /// let c = Inclusive.at(1.0).to(Inclusive.at(2.0));
    /// assert_eq!(a.iou(&a), 1.0);
    /// assert_eq!(a.iou(&b), 0.5);
    /// assert_eq!(a.iou(&c), 0.0);
    /// ```
    pub fn iou(&self, other: &Self) -> T {
        self.intersection(other)
            .map(|intersection| {
                let union = self.span(other);
                intersection.measure() / union.measure()
            })
            .unwrap_or(T::zero())
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

impl<T, L, R> Minimum<T> for Interval<T, L, R>
where
    LeftBounded<T, L>: Minimum<T>,
{
    fn minimum(&self) -> T {
        self.left.minimum()
    }
}

impl<T, L, R> Maximum<T> for Interval<T, L, R>
where
    RightBounded<T, R>: Maximum<T>,
{
    fn maximum(&self) -> T {
        self.right.maximum()
    }
}

/// ```
/// use kd_interval::{Interval, Exclusive, Inclusive, BoundType};
///
/// // Iterate Interval<i32, Exclusive, Inclusive>
/// let items: Vec<_> = Exclusive.at(0).to(Inclusive.at(10)).into_iter().collect();
/// assert_eq!(items.len(), 10);
/// assert_eq!(items[0], 1);
/// assert_eq!(items.last().unwrap(), &10);
///
/// // Iterate Interval<i32, BoundType, BoundType>
/// let items: Vec<_> = (BoundType::Exclusive.at(0).to(BoundType::Inclusive.at(10)))
///     .into_iter()
///     .collect();
/// assert_eq!(items.len(), 10);
/// assert_eq!(items[0], 1);
/// assert_eq!(items.last().unwrap(), &10);
/// ```
impl<T, L, R> IntoIterator for Interval<T, L, R>
where
    std::ops::RangeInclusive<T>: Iterator<Item = T>,
    Self: Minimum<T> + Maximum<T>,
{
    type Item = T;
    type IntoIter = std::ops::RangeInclusive<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.minimum()..=self.maximum()
    }
}
