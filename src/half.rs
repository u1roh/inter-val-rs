use ordered_float::{FloatCore, NotNan};

use crate::{boundary::Boundary, Bound, Exclusive, Inclusion, Inclusive, Maximum, Minimum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Left;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Right;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HalfBounded<T, B, Side>(Bound<T, B>, std::marker::PhantomData<Side>);

pub type LeftBounded<T, B> = HalfBounded<T, B, Left>;
pub type RightBounded<T, B> = HalfBounded<T, B, Right>;

impl<T, B, Side> std::ops::Deref for HalfBounded<T, B, Side> {
    type Target = Bound<T, B>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, B, Side> From<Bound<T, B>> for HalfBounded<T, B, Side> {
    fn from(b: Bound<T, B>) -> Self {
        HalfBounded(b, std::marker::PhantomData)
    }
}

impl<T, B: Boundary> Bound<T, B> {
    fn flip(self) -> Bound<T, B::Flip> {
        Bound {
            val: self.val,
            inclusion: self.inclusion.flip(),
        }
    }
}

impl<T: FloatCore, B: Boundary, Side> HalfBounded<NotNan<T>, B, Side> {
    pub fn closure(self) -> HalfBounded<NotNan<T>, Inclusive, Side> {
        Bound {
            val: self.val,
            inclusion: Inclusive,
        }
        .into()
    }
    pub fn interior(self) -> HalfBounded<NotNan<T>, Exclusive, Side> {
        Bound {
            val: self.val,
            inclusion: Exclusive,
        }
        .into()
    }
}

impl<T: Ord, B: Boundary> HalfBounded<T, B, Left> {
    pub fn includes(&self, other: &Self) -> bool {
        self.val <= other.val
    }
    pub fn contains(&self, t: &T) -> bool {
        self.inclusion.less(&self.val, t)
    }
    pub fn intersection(self, other: Self) -> Self {
        self.max(other)
    }
    pub fn union(self, other: Self) -> Self {
        self.min(other)
    }
    pub fn flip(self) -> HalfBounded<T, B::Flip, Right> {
        self.0.flip().into()
    }
}
impl<T: Ord, B: Boundary> HalfBounded<T, B, Right> {
    pub fn includes(&self, other: &Self) -> bool {
        other.val <= self.val
    }
    pub fn contains(&self, t: &T) -> bool {
        self.inclusion.less(t, &self.val)
    }
    pub fn intersection(self, other: Self) -> Self {
        self.min(other)
    }
    pub fn union(self, other: Self) -> Self {
        self.max(other)
    }
    pub fn flip(self) -> HalfBounded<T, B::Flip, Left> {
        self.0.flip().into()
    }
}

impl<T: FloatCore, B: Boundary> HalfBounded<NotNan<T>, B, Left> {
    pub fn inf(&self) -> NotNan<T> {
        self.val
    }
}
impl<T: FloatCore, B: Boundary> HalfBounded<NotNan<T>, B, Right> {
    pub fn sup(&self) -> NotNan<T> {
        self.val
    }
}

impl<T: Clone> Minimum<T> for HalfBounded<T, Inclusive, Left> {
    fn minimum(&self) -> T {
        self.val.clone()
    }
}
impl<T: Clone> Maximum<T> for HalfBounded<T, Inclusive, Right> {
    fn maximum(&self) -> T {
        self.val.clone()
    }
}

impl<T: num::Integer + Clone> Minimum<T> for HalfBounded<T, Exclusive, Left> {
    fn minimum(&self) -> T {
        self.val.clone() + T::one()
    }
}
impl<T: num::Integer + Clone> Maximum<T> for HalfBounded<T, Exclusive, Right> {
    fn maximum(&self) -> T {
        self.val.clone() - T::one()
    }
}

impl<T: num::Integer + Clone> Minimum<T> for HalfBounded<T, Inclusion, Left> {
    fn minimum(&self) -> T {
        match self.inclusion {
            Inclusion::Inclusive => self.val.clone(),
            Inclusion::Exclusive => self.val.clone() + T::one(),
        }
    }
}
impl<T: num::Integer + Clone> Maximum<T> for HalfBounded<T, Inclusion, Right> {
    fn maximum(&self) -> T {
        match self.inclusion {
            Inclusion::Inclusive => self.val.clone(),
            Inclusion::Exclusive => self.val.clone() - T::one(),
        }
    }
}
