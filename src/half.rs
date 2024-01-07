use crate::{
    bound_type::{Left, Right},
    traits::{BoundaryOf, Ceil, Flip, Floor, IntoGeneral},
    Bound, Exclusive, Inclusive,
};

#[derive(Debug, Clone, Copy)]
pub struct HalfBounded<T, B, LR>(pub(crate) Bound<T, B>, std::marker::PhantomData<LR>);

pub type LeftBounded<T, B> = HalfBounded<T, B, Left>;
pub type RightBounded<T, B> = HalfBounded<T, B, Right>;

impl<T, B, LR> std::ops::Deref for HalfBounded<T, B, LR> {
    type Target = Bound<T, B>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, B, LR> std::ops::DerefMut for HalfBounded<T, B, LR> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

mod ordering {
    use super::HalfBounded;
    use crate::traits::BoundaryOf;

    impl<T: PartialEq, B: PartialEq, LR> PartialEq for HalfBounded<T, B, LR> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<T: Eq, B: Eq, LR> Eq for HalfBounded<T, B, LR> {}

    impl<T: PartialOrd, B: BoundaryOf<LR>, LR> HalfBounded<T, B, LR> {
        fn ordering_key(&self) -> (&T, B::Ordered) {
            (&self.limit, self.bound_type.into_ordered())
        }
    }
    impl<T: PartialOrd, B: BoundaryOf<LR>, LR> PartialOrd for HalfBounded<T, B, LR> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.ordering_key().partial_cmp(&other.ordering_key())
        }
    }
}

impl<T, B, LR> From<Bound<T, B>> for HalfBounded<T, B, LR> {
    fn from(b: Bound<T, B>) -> Self {
        HalfBounded(b, std::marker::PhantomData)
    }
}

impl<T, B: IntoGeneral, LR> IntoGeneral for HalfBounded<T, B, LR> {
    type General = HalfBounded<T, B::General, LR>;
    fn into_general(self) -> Self::General {
        HalfBounded(self.0.into_general(), std::marker::PhantomData)
    }
}

impl<T, B: Flip, LR: Flip> Flip for HalfBounded<T, B, LR> {
    type Flip = HalfBounded<T, B::Flip, LR::Flip>;
    fn flip(self) -> Self::Flip {
        HalfBounded(self.0.flip(), std::marker::PhantomData)
    }
}

impl<T, B, LR> HalfBounded<T, B, LR> {
    pub fn cast<U: From<T>>(self) -> HalfBounded<U, B, LR> {
        self.0.cast().into()
    }
}
impl<T: num::NumCast, B, LR> HalfBounded<T, B, LR> {
    pub fn try_cast<U: num::NumCast>(self) -> Option<HalfBounded<U, B, LR>> {
        self.0.try_cast().map(Into::into)
    }
}

pub(crate) fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}
pub(crate) fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

impl<T: PartialOrd, B: BoundaryOf<LR>, LR> HalfBounded<T, B, LR> {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        partial_min(self, other)
    }
    fn max<'a>(&'a self, other: &'a Self) -> &'a Self {
        partial_max(self, other)
    }
}

impl<T: PartialOrd, B: BoundaryOf<Left>> LeftBounded<T, B> {
    pub fn includes(&self, other: &Self) -> bool {
        self.limit <= other.limit
    }
    pub fn contains(&self, t: &T) -> bool {
        self.bound_type.less(&self.limit, t)
    }
    pub fn intersection<'a>(&'a self, other: &'a Self) -> &'a Self {
        self.max(other)
    }
    pub fn union<'a>(&'a self, other: &'a Self) -> &'a Self {
        self.min(other)
    }

    pub fn hull(self, t: T) -> Self {
        Bound {
            limit: partial_min(self.0.limit, t),
            bound_type: self.0.bound_type,
        }
        .into()
    }

    pub fn dilate(self, delta: T) -> Self
    where
        T: std::ops::Sub<Output = T>,
    {
        Bound {
            limit: self.0.limit - delta,
            bound_type: self.0.bound_type,
        }
        .into()
    }

    pub fn inf(&self) -> &T {
        &self.limit
    }

    pub fn closure(self) -> LeftBounded<T, Inclusive> {
        Bound {
            limit: self.0.limit,
            bound_type: Inclusive,
        }
        .into()
    }

    pub fn interior(self) -> LeftBounded<T, Exclusive> {
        Bound {
            limit: self.0.limit,
            bound_type: Exclusive,
        }
        .into()
    }

    /// ```
    /// use kd_interval::{LeftBounded, Inclusive, Exclusive};
    /// let a: LeftBounded<_, _> = Inclusive.at(4).into();
    /// let b: LeftBounded<_, _> = Exclusive.at(4).into();
    /// let c: LeftBounded<_, _> = Inclusive.at(1.23).into();
    /// assert_eq!(a.ceil(), 4);
    /// assert_eq!(b.ceil(), 5);
    /// assert_eq!(c.ceil(), 2.0);
    /// ```
    pub fn ceil(&self) -> T
    where
        Bound<T, B>: Ceil<T>,
    {
        self.0.ceil()
    }
}

impl<T: PartialOrd, B: BoundaryOf<Right>> RightBounded<T, B> {
    pub fn includes(&self, other: &Self) -> bool {
        other.limit <= self.limit
    }
    pub fn contains(&self, t: &T) -> bool {
        self.bound_type.less(t, &self.limit)
    }
    pub fn intersection<'a>(&'a self, other: &'a Self) -> &'a Self {
        self.min(other)
    }
    pub fn union<'a>(&'a self, other: &'a Self) -> &'a Self {
        self.max(other)
    }

    pub fn hull(self, t: T) -> Self {
        Bound {
            limit: partial_max(self.0.limit, t),
            bound_type: self.0.bound_type,
        }
        .into()
    }

    pub fn dilate(self, delta: T) -> Self
    where
        T: std::ops::Add<Output = T>,
    {
        Bound {
            limit: self.0.limit + delta,
            bound_type: self.0.bound_type,
        }
        .into()
    }

    pub fn sup(&self) -> &T {
        &self.limit
    }

    pub fn closure(self) -> RightBounded<T, Inclusive> {
        Bound {
            limit: self.0.limit,
            bound_type: Inclusive,
        }
        .into()
    }

    /// ```
    /// use kd_interval::{RightBounded, Inclusive, Exclusive};
    /// let a : RightBounded<_, _> = Inclusive.at(7).into();
    /// let b : RightBounded<_, _> = Exclusive.at(7).into();
    /// let c : RightBounded<_, _> = Inclusive.at(4.56).into();
    /// assert_eq!(a.floor(), 7);
    /// assert_eq!(b.floor(), 6);
    /// assert_eq!(c.floor(), 4.0);
    /// ```
    pub fn floor(&self) -> T
    where
        Bound<T, B>: Floor<T>,
    {
        self.0.floor()
    }

    pub fn interior(self) -> RightBounded<T, Exclusive> {
        Bound {
            limit: self.0.limit,
            bound_type: Exclusive,
        }
        .into()
    }
}

impl<T, B> Floor<T> for RightBounded<T, B>
where
    Bound<T, B>: Floor<T>,
{
    fn floor(&self) -> T {
        self.0.floor()
    }
}
