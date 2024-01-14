use crate::bound_type::{Left, Right};
use crate::traits::{BoundaryOf, Flip, IntoGeneral};
use crate::{Bound, Exclusive, Inclusive, LeftBounded, RightBounded};

/// Return type of `Interval::union()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalUnion<T, L: Flip, R: Flip> {
    pub span: Interval<T, L, R>,
    pub gap: Option<Interval<T, R::Flip, L::Flip>>,
}
impl<T, L: Flip, R: Flip> IntervalUnion<T, L, R> {
    pub fn into_vec(self) -> Vec<Interval<T, L, R>> {
        self.into_iter().collect()
    }
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

/// Return type of `Interval::difference()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalDifference<T, L: Flip, R: Flip> {
    pub lower: Option<Interval<T, L, L::Flip>>,
    pub upper: Option<Interval<T, R::Flip, R>>,
}
impl<T, L: Flip<Flip = R>, R: Flip<Flip = L>> IntervalDifference<T, L, R> {
    pub fn into_vec(self) -> Vec<Interval<T, L, R>> {
        self.into_iter().collect()
    }
}
impl<T, L: Flip<Flip = R>, R: Flip<Flip = L>> IntoIterator for IntervalDifference<T, L, R> {
    type Item = Interval<T, L, R>;
    type IntoIter =
        std::iter::Chain<std::option::IntoIter<Self::Item>, std::option::IntoIter<Self::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        self.lower.into_iter().chain(self.upper)
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
/// * `T`: Numeric type bounding real number line. `T` should implements `PartialOrd`. `NaN` safety is not guaranteed when `T` is floating point type.
/// * `L`: Left boundary type. Specify one of [`Inclusive`], [`Exclusive`], or [`BoundType`](crate::BoundType).
/// * `R`: Right boundary type. Specify one of [`Inclusive`] [`Exclusive`], or [`BoundType`](crate::BoundType).
/// * `Interval<T>` (= `Interval<T, Inclusive, Inclusive>`) represents a closed interval, i.e., *[a, b]*.
/// * `Interval<T, Exclusive>` (= `Interval<T, Exclusive, Exclusive>`) represents a open interval, i.e., *(a, b)*.
/// * `Interval<T, Inclusive, Exclusive>` represents a right half-open interval, i.e., *[a, b)*.
/// * `Interval<T, Exclusive, Inclusive>` represents a left half-open interval, i.e., *(a, b]*.
/// * `Interval<T, BoundType>` represents any of the above.
///
/// This type is considered as an interval on ℝ (real number line), even if an integer type is specified for `T`.
///
/// # Memory cost
/// ```
/// use inter_val::{Interval, Exclusive, Inclusive, BoundType};
/// use std::mem::size_of;
///
/// // When bound type is statically determined, the size of the interval is just the size of two `T`.
/// assert_eq!(size_of::<Interval<i32, Inclusive>>(), size_of::<i32>() * 2);
/// assert_eq!(size_of::<Interval<f64, Exclusive>>(), size_of::<f64>() * 2);
///
/// // Size is larger when bound type is not statically determined.
/// assert!(size_of::<Interval<i32, BoundType>>() >= (size_of::<i32>() + size_of::<BoundType>()) * 2);
/// ```
///
/// # Properties
/// ```txt
/// lower_bound     left              . center          right    upper_bound
/// ...------------>|<------- self -------------------->|<------------ ...
///                 inf                                 sup
///                 [<------------ closure ------------>]
///                  (<----------- interior ---------->)
/// ```
///
/// # Set operations
/// ```txt
/// |<------------- a ----------------->|   . p           |<-------- c -------->|
///        |<--------------- b ------------------->|
///        |<--- a.intersection(&b) --->|
///                                     |<-- a.gap(&c) -->|
/// |<------------- a.hull(p) ------------->|
/// |<---------------------------------- a.span(&c) --------------------------->|
/// |<--------------------------------->|        +        |<------------------->| a.union(&c)
/// |<---->| a.difference(&b)
///                                                |<- δ -+---- c.dilate(δ) ----+- δ ->|
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T, L = Inclusive, R = L> {
    pub(crate) left: LeftBounded<T, L>,
    pub(crate) right: RightBounded<T, R>,
}

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
    /// use inter_val::{Interval, BoundType, Exclusive, Inclusive};
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
    /// use inter_val::{Interval, BoundType, Exclusive, Inclusive};
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
    /// # use inter_val::{Interval, Exclusive, Inclusive};
    /// Interval::new(Inclusive.at(3), Exclusive.at(0)); // [3, 0) is empty.
    /// ```
    /// ```should_panic
    /// # use inter_val::{Interval, Exclusive, Inclusive};
    /// Interval::new(Inclusive.at(3), Exclusive.at(3)); // [3, 3) is empty.
    /// ```
    pub fn new(left: Bound<T, L>, right: Bound<T, R>) -> Self {
        Self::try_new(left, right).expect("Invalid interval: left must be less than right.")
    }

    /// ```
    /// use inter_val::{Interval, Exclusive, Inclusive};
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
    /// use inter_val::{Interval, Exclusive, Inclusive};
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
    /// # use inter_val::{Interval, Exclusive, Inclusive};
    /// Interval::<i32, Inclusive, Exclusive>::between(1, 1); // Panics since [1, 1) is empty.
    /// ```
    pub fn between(a: T, b: T) -> Self
    where
        T: Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        Self::try_between(a, b).unwrap()
    }

    /// Shorthand of `.left().limit`
    /// ```
    /// use inter_val::{Interval, Exclusive, Inclusive};
    /// let a = Interval::new(Exclusive.at(-1.0), Inclusive.at(1.0));
    /// assert_eq!(a.inf(), &-1.0);
    /// assert_eq!(a.inf(), &a.left().limit);
    /// assert!(!a.contains(&-1.0));
    /// ```
    pub fn inf(&self) -> &T {
        self.left.inf()
    }

    /// Shorthand of `.right().limit`
    /// ```
    /// use inter_val::{Interval, Exclusive, Inclusive};
    /// let a = Interval::new(Inclusive.at(-1.0), Exclusive.at(1.0));
    /// assert_eq!(a.sup(), &1.0);
    /// assert_eq!(a.sup(), &a.right().limit);
    /// assert!(!a.contains(&1.0));
    /// ```
    pub fn sup(&self) -> &T {
        self.right.sup()
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

    /// ```
    /// use inter_val::{Interval, Inclusive, Exclusive};
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
    /// use inter_val::{Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Exclusive.at(7));    // [4, 7)
    /// assert_eq!(a.dilate(2), Inclusive.at(2).to(Exclusive.at(9)));   // [4-2, 7+2) = [2, 9)
    /// assert_eq!(a.dilate(-1), Inclusive.at(5).to(Exclusive.at(6)));  // [4+1, 7-1) = [5, 6)
    /// ```
    /// ```should_panic
    /// use inter_val::{Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Exclusive.at(7));    // [4, 7)
    /// a.dilate(-2);   // panic! [4+2, 7-2) = [6, 5) is empty.
    /// ```
    pub fn dilate(self, delta: T) -> Self
    where
        T: Clone + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
    {
        Self::new_(self.left.dilate(delta.clone()), self.right.dilate(delta)).unwrap()
    }

    /// ```
    /// use inter_val::{Interval, Inclusive, Exclusive};
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
    /// use inter_val::{Interval, Inclusive, Exclusive};
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
    /// use inter_val::{Interval, Inclusive, Exclusive};
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
    /// use inter_val::{Interval, Inclusive, Exclusive};
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
    /// use inter_val::{Interval, Inclusive, Exclusive};
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
    /// use inter_val::{Interval, Inclusive, Exclusive};
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
    /// use inter_val::{Interval, Inclusive, Exclusive};
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
    /// use inter_val::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(2.1).to(Inclusive.at(5.3));
    /// assert_eq!(a.measure(), 5.3 - 2.1);
    ///
    /// let a = Exclusive.at(0).to(Exclusive.at(1));    // (0, 1)
    /// assert_eq!(a.measure(), 1);
    /// ```
    pub fn measure(&self) -> T
    where
        T: Clone + std::ops::Sub<Output = T>,
    {
        self.sup().clone() - self.inf().clone()
    }

    /// ```
    /// use inter_val::{Inclusive, Exclusive};
    /// let a = Exclusive.at(10).to(Inclusive.at(20)); // (10, 20]
    /// assert!(a.step_by(2).eq(vec![12, 14, 16, 18, 20]));
    /// ```
    pub fn step_by(&self, step: T) -> impl Iterator<Item = T> + '_
    where
        T: Clone,
        for<'a> T: std::ops::AddAssign<&'a T>,
    {
        self.left
            .step_by(step)
            .take_while(|t| self.right.contains(t))
    }

    /// ```
    /// use inter_val::{Inclusive, Exclusive};
    /// let a = Exclusive.at(10).to(Inclusive.at(20)); // (10, 20]
    /// assert!(a.step_rev_by(2).eq(vec![20, 18, 16, 14, 12]));
    /// ```
    pub fn step_rev_by(&self, step: T) -> impl Iterator<Item = T> + '_
    where
        T: Clone,
        for<'a> T: std::ops::SubAssign<&'a T>,
    {
        self.right
            .step_rev_by(step)
            .take_while(|t| self.left.contains(t))
    }

    /// ```
    /// use inter_val::{Interval, Inclusive, Exclusive, Nullable};
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
    /// use inter_val::{Interval, Nullable};
    /// let hull = Interval::<_>::hull_many(vec![3, 9, 2, 5]).unwrap(); // [2, 9]
    /// assert_eq!(hull.inf(), &2);
    /// assert_eq!(hull.sup(), &9);
    ///
    /// let hull = Interval::<_>::hull_many(vec![3.1, 9.2, 2.3, 5.4]).unwrap(); // [2.3, 9.2]
    /// assert_eq!(hull.inf(), &2.3);
    /// assert_eq!(hull.sup(), &9.2);
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

impl<T: PartialOrd, L: BoundaryOf<Left, Flip = R>, R: BoundaryOf<Right, Flip = L>>
    Interval<T, L, R>
{
    /// Difference is defined only for `Interval<T, Inclusive, Exclusive>`, `Interval<T, Exclusive, Inclusive>`, and `Interval<T, BoundType>`.
    /// ```
    /// use inter_val::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));
    /// let b = Inclusive.at(1).to(Exclusive.at(4));
    /// let diff = a.difference(&b);
    /// assert!(diff.lower.is_some() && diff.upper.is_none());
    /// assert_eq!(diff.lower.unwrap(), Inclusive.at(0).to(Exclusive.at(1)));
    /// assert_eq!(diff.into_iter().collect::<Vec<_>>().len(), 1);
    /// ```
    pub fn difference(&self, other: &Self) -> IntervalDifference<T, L, R>
    where
        T: Clone,
    {
        IntervalDifference {
            lower: Self::new_(self.left.clone(), other.lower_bound()),
            upper: Self::new_(other.upper_bound(), self.right.clone()),
        }
    }
}

impl<T: PartialOrd + Clone> Interval<T, Inclusive, Exclusive> {
    /// ```
    /// use inter_val::{Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));    // [0, 3)
    /// let (b, c) = a.try_split_at(1); // [0, 1) and [1, 3)
    /// assert_eq!(b, Some(Inclusive.at(0).to(Exclusive.at(1))));
    /// assert_eq!(c, Some(Inclusive.at(1).to(Exclusive.at(3))));
    ///
    /// let (b, c) = a.try_split_at(0);
    /// assert_eq!(b, None);    // [0, 0) is empty.
    /// assert_eq!(c, Some(a)); // [0, 3)
    /// ```
    pub fn try_split_at(&self, t: T) -> (Option<Self>, Option<Self>) {
        if !self.left.contains(&t) {
            return (None, Some(self.clone()));
        }
        if !self.right.contains(&t) {
            return (Some(self.clone()), None);
        }
        let lower = Self::new_(self.left.clone(), Exclusive.at(t.clone()).into());
        let upper = Self::new_(Inclusive.at(t).into(), self.right.clone());
        (lower, upper)
    }

    /// ```
    /// use inter_val::{Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));    // [0, 3)
    /// let (b, c) = a.split_at(1); // [0, 1) and [1, 3)
    /// assert_eq!(b, Inclusive.at(0).to(Exclusive.at(1)));
    /// assert_eq!(c, Inclusive.at(1).to(Exclusive.at(3)));
    /// ```
    /// ```should_panic
    /// use inter_val::{Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3));    // [0, 3)
    /// let (b, c) = a.split_at(0);
    /// ```
    pub fn split_at(&self, t: T) -> (Self, Self) {
        assert!(self.contains(&t));
        let lower = Self::new_(self.left.clone(), Exclusive.at(t.clone()).into());
        let upper = Self::new_(Inclusive.at(t).into(), self.right.clone());
        (lower.unwrap(), upper.unwrap())
    }
}

impl<T: num::Float, L: BoundaryOf<Left>, R: BoundaryOf<Right>> Interval<T, L, R> {
    /// ```
    /// use inter_val::{Interval, Inclusive};
    /// let a = Inclusive.at(2.1).to(Inclusive.at(5.3));
    /// assert_eq!(a.center(), (2.1 + 5.3) / 2.0);
    /// ```
    pub fn center(&self) -> T {
        (self.left.limit + self.right.limit) / (T::one() + T::one())
    }

    /// IoU - Intersection over Union.
    /// ```
    /// use inter_val::{Interval, Inclusive};
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

    /// Linear interpolation.
    /// ```
    /// use inter_val::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(2.0).to(Inclusive.at(4.0));    // [2, 4]
    /// assert_eq!(a.lerp(0.0), 2.0);
    /// assert_eq!(a.lerp(0.5), 3.0);
    /// assert_eq!(a.lerp(1.0), 4.0);
    /// assert_eq!(a.lerp(1.1), 4.2);
    /// ```
    pub fn lerp(&self, zero_to_one: T) -> T {
        (T::one() - zero_to_one) * *self.inf() + zero_to_one * *self.sup()
    }

    /// ```
    /// use inter_val::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(2.0).to(Inclusive.at(4.0));    // [2, 4]
    /// let b = Inclusive.at(2.0).to(Exclusive.at(4.0));    // [2, 4)
    /// let c = Exclusive.at(2.0).to(Inclusive.at(4.0));    // (2, 4]
    /// assert!(a.step_uniform(4).eq(vec![2.0, 2.5, 3.0, 3.5, 4.0]));
    /// assert!(b.step_uniform(4).eq(vec![2.0, 2.5, 3.0, 3.5]));
    /// assert!(c.step_uniform(4).eq(vec![2.5, 3.0, 3.5, 4.0]));
    /// ```
    pub fn step_uniform(&self, n: usize) -> impl Iterator<Item = T> + '_ {
        let step = self.measure() / T::from(n).unwrap();
        let (mut i, mut t) = if self.left.bound_type.is_inclusive() {
            (0, *self.inf())
        } else {
            (1, *self.inf() + step)
        };
        let last = if self.right.bound_type.is_inclusive() {
            n
        } else {
            n - 1
        };
        std::iter::from_fn(move || {
            let ret = (i <= last).then_some(t);
            t = if i == n { *self.sup() } else { t + step };
            i += 1;
            ret
        })
    }
}

impl<T, L, R> Interval<T, L, R> {
    /// Cast by `From<T>`.
    /// ```
    /// use inter_val::{Interval, Exclusive};
    /// let src: Interval<i32, Exclusive> = Interval::between(0, 1);  // open interval (0, 1)
    /// let dst = src.cast::<f64>();
    /// assert!(dst.contains(&0.5));
    /// ```
    pub fn cast<U: From<T>>(self) -> Interval<U, L, R> {
        Interval {
            left: self.left.cast(),
            right: self.right.cast(),
        }
    }
}

impl<T: num::NumCast, L, R> Interval<T, L, R> {
    /// Cast by `num::NumCast`.
    /// ```
    /// use inter_val::{Interval, Exclusive};
    /// let src: Interval<f64> = Interval::between(1.2, 7.8);  // closed interval [1.2, 7.8]
    /// let dst = src.try_cast::<i32>().unwrap();
    /// assert_eq!(dst.inf(), &1);
    /// assert_eq!(dst.sup(), &7);
    /// ```
    pub fn try_cast<U: num::NumCast>(self) -> Option<Interval<U, L, R>> {
        Some(Interval {
            left: self.left.try_cast()?,
            right: self.right.try_cast()?,
        })
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

/// ```
/// use inter_val::{Interval, Exclusive, Inclusive, BoundType};
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
    T: num::Integer + Clone,
    L: BoundaryOf<Left>,
    R: BoundaryOf<Right>,
    for<'a> T: std::ops::AddAssign<&'a T> + std::ops::SubAssign<&'a T>,
{
    type Item = T;
    type IntoIter = std::ops::RangeInclusive<T>;
    fn into_iter(self) -> Self::IntoIter {
        let first = self.left.step_by(T::one()).next().unwrap();
        let last = self.right.step_rev_by(T::one()).next().unwrap();
        first..=last
    }
}
