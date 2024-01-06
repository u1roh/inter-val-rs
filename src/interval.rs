use ordered_float::FloatCore;

use crate::bound_type::{Left, Right};
use crate::traits::{BoundaryOf, Flip, IntoGeneral, Maximum, Minimum};
use crate::{Bound, Exclusive, Inclusive, LeftBounded, RightBounded};

/// Return type of `Interval::union()`.
pub struct IntervalUnion<T, L: Flip, R: Flip> {
    pub hull: Interval<T, L, R>,
    pub gap: Option<Interval<T, R::Flip, L::Flip>>,
}
impl<T, L: Flip, R: Flip> IntoIterator for IntervalUnion<T, L, R> {
    type Item = Interval<T, L, R>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        if let Some(gap) = self.gap {
            let first = Interval {
                left: self.hull.left,
                right: gap.left.flip(),
            };
            let second = Interval {
                left: gap.right.flip(),
                right: self.hull.right,
            };
            vec![first, second].into_iter()
        } else {
            vec![self.hull].into_iter()
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

/// Interval like *[a, b]*, *(a, b)*, *[a, b)*, and *(a, b]* for any `Ord` type.
/// * `T`: Scalar type. `T` should implements `Ord`. Use [`T`](crate::ordered_float::NotNan) for floating point numbers.
/// * `L`: Left boundary type. Specify one of [`Inclusive`], [`Exclusive`], or [`BoundType`](crate::BoundType).
/// * `R`: Right boundary type. Specify one of [`Inclusive`] [`Exclusive`], or [`BoundType`](crate::BoundType).
/// * `Interval<T, Inclusive>` represents a closed interval, i.e., *[a, b]*.
/// * `Interval<T, Exclusive>` represents a open interval, i.e., *(a, b)*.
/// * `Interval<T, Inclusive, Exclusive>` represents a right half-open interval, i.e., *[a, b)*.
/// * `Interval<T, Exclusive, Inclusive>` represents a left half-open interval, i.e., *(a, b]*.
/// * `Interval<T>` (= `Interval<T, BoundType, BoundType>`) represents any of the above.
///
/// ```
/// use kd_interval::{Interval, Exclusive, Inclusive, BoundType};
/// assert_eq!(std::mem::size_of::<Interval<i32, Inclusive>>(), std::mem::size_of::<i32>() * 2);
/// assert_eq!(std::mem::size_of::<Interval<f64, Exclusive>>(), std::mem::size_of::<f64>() * 2);
/// assert!(std::mem::size_of::<Interval<i32>>() > (std::mem::size_of::<i32>() + std::mem::size_of::<BoundType>()) * 2);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Interval<T, L = crate::BoundType, R = L> {
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

    /// Create a new interval.
    /// ```
    /// use std::any::{Any, TypeId};
    /// use kd_interval::{Interval, BoundType, Exclusive, Inclusive};
    ///
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::new(0.into(), 3.into()).unwrap();
    /// assert!(a.contains(&0));
    /// assert!(a.contains(&2));
    /// assert!(!a.contains(&3));
    ///
    /// let a = Interval::new(Exclusive.at(0), Inclusive.at(3)).unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Exclusive, Inclusive>>());
    ///
    /// let a = Interval::new(BoundType::Exclusive.at(0), BoundType::Exclusive.at(3)).unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, BoundType, BoundType>>());
    ///
    /// assert!(Interval::new(Inclusive.at(3), Exclusive.at(0)).is_none());
    /// assert!(Interval::new(Inclusive.at(3), Exclusive.at(3)).is_none());
    /// assert!(Interval::new(Inclusive.at(3), Inclusive.at(3)).is_some());
    /// ```
    pub fn new(left: Bound<T, L>, right: Bound<T, R>) -> Option<Self> {
        Self::new_(left.into(), right.into())
    }

    // /// ```
    // /// use kd_interval::{Interval, Exclusive, Inclusive};
    // /// let a = Interval::try_new(Inclusive.at(-1.0), Exclusive.at(1.0)).unwrap();
    // /// assert!(a.contains(&-1.0));
    // /// assert!(!a.contains(&1.0));
    // ///
    // /// let a = Interval::<_, Exclusive, Inclusive>::try_new(1.23.into(), 4.56.into())
    // ///     .unwrap()
    // ///     .unwrap();
    // /// assert!(!a.contains(&1.23));
    // /// assert!(a.contains(&1.23000000000001));
    // /// assert!(a.contains(&4.56));
    // /// ```
    // pub fn try_new<T2>(left: Bound<T2, L>, right: Bound<T2, R>) -> Result<Option<Self>, T::Error>
    // where
    //     T: Scalar<T2>,
    // {
    //     let left = Bound {
    //         limit: T::scalar_try_from(left.limit)?,
    //         bound_type: left.bound_type,
    //     };
    //     let right = Bound {
    //         limit: T::scalar_try_from(right.limit)?,
    //         bound_type: right.bound_type,
    //     };
    //     Ok(Self::new(left, right))
    // }

    /// ```
    /// use kd_interval::{Interval, Exclusive, Inclusive};
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::between(-2, 5).unwrap();
    /// assert_eq!(a, Inclusive.at(-2).to(Exclusive.at(5)).unwrap());
    /// ```
    pub fn between(left: T, right: T) -> Option<Self>
    where
        T: Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        Self::new(left.into(), right.into())
    }

    // /// ```
    // /// use kd_interval::{Interval, Exclusive, Inclusive};
    // /// let a: Interval<f64, Inclusive, Exclusive> = Interval::try_between(-1.0, 1.0).unwrap();
    // /// assert_eq!(a, Inclusive.at(-1.0).to(Exclusive.at(1.0)).unwrap());
    // /// ```
    // pub fn try_between<T2>(left: T2, right: T2) -> Result<Option<Self>, T::Error>
    // where
    //     T: Scalar<T2> + Into<Bound<T, L>> + Into<Bound<T, R>>,
    // {
    //     Ok(Self::new(
    //         T::scalar_try_from(left)?.into(),
    //         T::scalar_try_from(right)?.into(),
    //     ))
    // }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Inclusive.at(7)).unwrap();
    /// let b = Exclusive.at(4).to(Inclusive.at(7)).unwrap();
    /// let c = Inclusive.at(1.23).to(Inclusive.at(4.56)).unwrap();
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
    /// let a = Inclusive.at(4).to(Inclusive.at(7)).unwrap();
    /// let b = Inclusive.at(4).to(Exclusive.at(7)).unwrap();
    /// let c = Inclusive.at(1.23).to(Inclusive.at(4.56)).unwrap();
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
    /// let a = Inclusive.at(4).to(Exclusive.at(7)).unwrap();
    /// let b = Exclusive.at(1.23).to(Inclusive.at(4.56)).unwrap();
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
    /// let a = Inclusive.at(4).to(Exclusive.at(7)).unwrap();
    /// assert_eq!(a.dilate(2), Inclusive.at(2).to(Exclusive.at(9)).unwrap());
    /// assert_eq!(a.dilate(-1), Inclusive.at(5).to(Exclusive.at(6)).unwrap());
    /// assert!(std::panic::catch_unwind(|| a.dilate(-2)).is_err());
    /// ```
    pub fn dilate(self, delta: T) -> Self
    where
        T: Clone + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
    {
        Self::new_(self.left.dilate(delta.clone()), self.right.dilate(delta)).unwrap()
    }

    // /// ```
    // /// use kd_interval::{Inclusive, Exclusive};
    // /// let a = Inclusive.at(0.0).to(Exclusive.at(10.0)).unwrap();
    // /// assert_eq!(a.try_dilate(2.0).unwrap(), Inclusive.at(-2.0).to(Exclusive.at(12.0)).unwrap());
    // /// assert_eq!(a.try_dilate(-1.5).unwrap(), Inclusive.at(1.5).to(Exclusive.at(8.5)).unwrap());
    // /// assert!(a.try_dilate(-6.0).is_err());
    // /// ```
    // pub fn try_dilate<X>(self, delta: X) -> Result<Self, crate::Error>
    // where
    //     T: Scalar<X>,
    //     crate::Error: From<T::Error>,
    //     X: Clone + std::ops::Add<Output = X> + std::ops::Sub<Output = X>,
    // {
    //     Self::new_(
    //         self.left.try_dilate(delta.clone())?,
    //         self.right.try_dilate(delta)?,
    //     )
    //     .ok_or(crate::IntervalIsEmpty.into())
    // }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(0).to(Exclusive.at(4)).unwrap();
    /// let c = Inclusive.at(1).to(Exclusive.at(4)).unwrap();
    /// assert!(a.includes(&a));
    /// assert!(!a.includes(&b) && b.includes(&a));
    /// assert!(!a.includes(&c) && !c.includes(&a));
    /// ```
    pub fn includes(&self, other: &Self) -> bool {
        self.left.includes(&other.left) && self.right.includes(&other.right)
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(1).to(Exclusive.at(4)).unwrap();
    /// let c = Inclusive.at(3).to(Exclusive.at(5)).unwrap();
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
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(1).to(Exclusive.at(4)).unwrap();
    /// let c = Inclusive.at(3).to(Exclusive.at(5)).unwrap();
    /// assert_eq!(a.intersection(a), Some(a));
    /// assert_eq!(a.intersection(b), Inclusive.at(1).to(Exclusive.at(3)));
    /// assert_eq!(a.intersection(c), None);
    /// ```
    pub fn intersection(self, other: Self) -> Option<Self> {
        Self::new_(
            self.left.intersection(other.left),
            self.right.intersection(other.right),
        )
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(5).to(Exclusive.at(8)).unwrap();
    /// assert_eq!(a.hull(b), Inclusive.at(0).to(Exclusive.at(8)).unwrap());
    /// ```
    pub fn hull(self, other: Self) -> Self {
        Self {
            left: self.left.union(other.left),
            right: self.right.union(other.right),
        }
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(5).to(Exclusive.at(8)).unwrap();
    /// assert_eq!(a.gap(b), Inclusive.at(3).to(Exclusive.at(5)));
    /// ```
    pub fn gap(self, other: Self) -> Option<Interval<T, R::Flip, L::Flip>>
    where
        L::Flip: BoundaryOf<Right>,
        R::Flip: BoundaryOf<Left>,
    {
        Interval::new_(self.right.flip(), other.left.flip())
            .or(Interval::new_(other.right.flip(), self.left.flip()))
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(5).to(Exclusive.at(8)).unwrap();
    /// let union = a.union(b);
    /// assert_eq!(union.hull, a.hull(b));
    /// assert_eq!(union.gap, a.gap(b));
    /// let union_ints: Vec<Interval<_, _, _>> = union.into_iter().collect();
    /// assert_eq!(union_ints.len(), 2);
    /// assert_eq!(union_ints[0], a);
    /// assert_eq!(union_ints[1], b);
    /// ```
    pub fn union(self, other: Self) -> IntervalUnion<T, L, R>
    where
        T: Clone,
        L::Flip: BoundaryOf<Right>,
        R::Flip: BoundaryOf<Left>,
    {
        IntervalUnion {
            gap: self.clone().gap(other.clone()),
            hull: self.hull(other),
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
    /// use kd_interval::Interval;
    /// let span = Interval::enclosure_of_items(vec![3, 9, 2, 5]).unwrap(); // [2, 9]
    /// assert_eq!(span.min(), 2);
    /// assert_eq!(span.max(), 9);
    /// ```
    pub fn enclosure_of_items<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.hull(item.into())))
    }
}

impl<T: FloatCore, L: BoundaryOf<Left>, R: BoundaryOf<Right>> Interval<T, L, R> {
    // /// ```
    // /// use kd_interval::{Interval, Exclusive, Inclusive};
    // /// let a = Interval::new(Inclusive.at(-1.0), Exclusive.at(1.0)).unwrap();
    // /// assert!(a.contains(&-1.0));
    // /// assert!(!a.contains(&1.0));
    // /// ```
    // pub fn new(left: Bound<T, L>, right: Bound<T, R>) -> Result<Option<Self>, FloatIsNan> {
    //     Self::new(left, right)
    // }

    // /// ```
    // /// use kd_interval::{Interval, Exclusive, Inclusive};
    // /// let a: Interval<_, Inclusive, Exclusive> = Interval::float_between(-1.0, 1.0).unwrap();
    // /// assert!(a.contains(&-1.0));
    // /// assert!(!a.contains(&1.0));
    // /// ```
    // pub fn float_between(left: T, right: T) -> Result<Option<Self>, FloatIsNan>
    // where
    //     T: Into<Bound<T, L>> + Into<Bound<T, R>>,
    // {
    //     Self::new(left.into(), right.into())
    // }

    /// ```
    /// use kd_interval::{Interval, Exclusive, Inclusive};
    /// let a = Interval::new(Inclusive.at(-1.0), Inclusive.at(1.0)).unwrap();
    /// assert_eq!(a.inf(), -1.0);
    /// assert!(a.contains(&-1.0));
    ///
    /// let b = Interval::new(Exclusive.at(-1.0), Inclusive.at(1.0)).unwrap();
    /// assert_eq!(b.inf(), -1.0);
    /// assert!(!b.contains(&-1.0));
    /// ```
    pub fn inf(&self) -> T {
        self.left.inf()
    }

    /// ```
    /// use kd_interval::{Interval, Exclusive, Inclusive};
    /// let a = Interval::new(Inclusive.at(-1.0), Inclusive.at(1.0)).unwrap();
    /// assert_eq!(a.sup(), 1.0);
    /// assert!(a.contains(&1.0));
    ///
    /// let b = Interval::new(Inclusive.at(-1.0), Exclusive.at(1.0)).unwrap();
    /// assert_eq!(b.sup(), 1.0);
    /// assert!(!b.contains(&1.0));
    /// ```
    pub fn sup(&self) -> T {
        self.right.sup()
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive};
    /// let a = Inclusive.at(2.1).to(Inclusive.at(5.3)).unwrap();
    /// assert_eq!(a.measure(), 5.3 - 2.1);
    ///
    /// let a = Inclusive.at(std::f64::INFINITY).to(Inclusive.at(std::f64::INFINITY)).unwrap();
    /// assert!(a.measure().is_nan());
    /// ```
    pub fn measure(&self) -> T {
        self.right.limit - self.left.limit
    }

    /// ```
    /// use kd_interval::{Interval, Inclusive};
    /// let a = Inclusive.at(2.1).to(Inclusive.at(5.3)).unwrap();
    /// assert_eq!(a.center(), (2.1 + 5.3) / 2.0);
    ///
    /// let a = Inclusive.at(std::f64::NEG_INFINITY).to(Inclusive.at(std::f64::INFINITY)).unwrap();
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
    /// let a = Inclusive.at(0.0).to(Inclusive.at(1.0)).unwrap();
    /// let b = Inclusive.at(0.0).to(Inclusive.at(2.0)).unwrap();
    /// let c = Inclusive.at(1.0).to(Inclusive.at(2.0)).unwrap();
    /// assert_eq!(a.iou(a), 1.0);
    /// assert_eq!(a.iou(b), 0.5);
    /// assert_eq!(a.iou(c), 0.0);
    /// ```
    pub fn iou(self, other: Self) -> T {
        self.intersection(other)
            .map(|intersection| {
                let union = self.hull(other);
                intersection.measure() / union.measure()
            })
            .unwrap_or(T::zero())
    }
}

impl<T: FloatCore> Interval<T, Inclusive, Inclusive> {
    /// ```
    /// use kd_interval::Interval;
    /// let span = Interval::enclosure_of_floats(vec![3.1, 9.2, 2.3, 5.4]).unwrap(); // [2.3, 9.2]
    /// assert_eq!(span.inf(), 2.3);
    /// assert_eq!(span.sup(), 9.2);
    /// assert!(Interval::<f64, _, _>::enclosure_of_floats(vec![]).is_none());
    /// ```
    pub fn enclosure_of_floats(floats: impl IntoIterator<Item = T>) -> Option<Self> {
        Self::enclosure_of_items(floats)
        // let mut inf = T::infinity();
        // let mut sup = T::neg_infinity();
        // for x in floats {
        //     let x = NotNan::new(x)?;
        //     inf = inf.min(x);
        //     sup = sup.max(x);
        // }
        // Ok(Self::between(inf, sup))
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
/// let items: Vec<_> = Exclusive.at(0).to(Inclusive.at(10)).unwrap().into_iter().collect();
/// assert_eq!(items.len(), 10);
/// assert_eq!(items[0], 1);
/// assert_eq!(items.last().unwrap(), &10);
///
/// // Iterate Interval<i32, BoundType, BoundType>
/// let items: Vec<_> = (BoundType::Exclusive.at(0).to(BoundType::Inclusive.at(10)))
///     .unwrap()
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
