use ordered_float::{FloatCore, NotNan};

use crate::{boundary::Boundary, Bound, Exclusive, Inclusion, Inclusive, Maximum, Minimum};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftInclusion<B>(B);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RightInclusion<B>(B);

impl<B: Boundary> Boundary for LeftInclusion<B> {
    type Flip = RightInclusion<B::Flip>;
    fn flip(self) -> Self::Flip {
        RightInclusion(self.0.flip())
    }
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool {
        self.0.less(this, t)
    }
}
impl<B: Boundary> Boundary for RightInclusion<B> {
    type Flip = LeftInclusion<B::Flip>;
    fn flip(self) -> Self::Flip {
        LeftInclusion(self.0.flip())
    }
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool {
        self.0.less(this, t)
    }
}

macro_rules! impl_ord {
    (($lhs:ident, $rhs:ident): $type:ty => $body:expr) => {
        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for $type {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                let $lhs = self;
                let $rhs = other;
                $body
            }
        }
    };
}

impl_ord!((_lhs, _rhs): LeftInclusion<Inclusive> => std::cmp::Ordering::Equal);
impl_ord!((_lhs, _rhs): LeftInclusion<Exclusive> => std::cmp::Ordering::Equal);
impl_ord!((_lhs, _rhs): RightInclusion<Inclusive> => std::cmp::Ordering::Equal);
impl_ord!((_lhs, _rhs): RightInclusion<Exclusive> => std::cmp::Ordering::Equal);
impl_ord!((lhs, rhs): LeftInclusion<Inclusion> => match (lhs.0, rhs.0) {
    (Inclusion::Inclusive, Inclusion::Inclusive) => std::cmp::Ordering::Equal,
    (Inclusion::Inclusive, Inclusion::Exclusive) => std::cmp::Ordering::Less,
    (Inclusion::Exclusive, Inclusion::Inclusive) => std::cmp::Ordering::Greater,
    (Inclusion::Exclusive, Inclusion::Exclusive) => std::cmp::Ordering::Equal,
});
impl_ord!((lhs, rhs): RightInclusion<Inclusion> => match (lhs.0, rhs.0) {
    (Inclusion::Inclusive, Inclusion::Inclusive) => std::cmp::Ordering::Equal,
    (Inclusion::Inclusive, Inclusion::Exclusive) => std::cmp::Ordering::Greater,
    (Inclusion::Exclusive, Inclusion::Inclusive) => std::cmp::Ordering::Less,
    (Inclusion::Exclusive, Inclusion::Exclusive) => std::cmp::Ordering::Equal,
});

pub type LeftBounded<T, B> = Bound<T, LeftInclusion<B>>;
pub type RightBounded<T, B> = Bound<T, RightInclusion<B>>;

impl<T, B> From<Bound<T, B>> for LeftBounded<T, B> {
    fn from(b: Bound<T, B>) -> Self {
        Self {
            val: b.val,
            inclusion: LeftInclusion(b.inclusion),
        }
    }
}
impl<T, B> From<Bound<T, B>> for RightBounded<T, B> {
    fn from(b: Bound<T, B>) -> Self {
        Self {
            val: b.val,
            inclusion: RightInclusion(b.inclusion),
        }
    }
}

impl<T> From<LeftBounded<T, Inclusive>> for LeftBounded<T, Inclusion> {
    fn from(src: LeftBounded<T, Inclusive>) -> Self {
        Self {
            val: src.val,
            inclusion: LeftInclusion(src.inclusion.0.into()),
        }
    }
}
impl<T> From<LeftBounded<T, Exclusive>> for LeftBounded<T, Inclusion> {
    fn from(src: LeftBounded<T, Exclusive>) -> Self {
        Self {
            val: src.val,
            inclusion: LeftInclusion(src.inclusion.0.into()),
        }
    }
}
impl<T> From<RightBounded<T, Inclusive>> for RightBounded<T, Inclusion> {
    fn from(src: RightBounded<T, Inclusive>) -> Self {
        Self {
            val: src.val,
            inclusion: RightInclusion(src.inclusion.0.into()),
        }
    }
}
impl<T> From<RightBounded<T, Exclusive>> for RightBounded<T, Inclusion> {
    fn from(src: RightBounded<T, Exclusive>) -> Self {
        Self {
            val: src.val,
            inclusion: RightInclusion(src.inclusion.0.into()),
        }
    }
}

impl<T, B: Boundary> Bound<T, B> {
    pub fn flip(self) -> Bound<T, B::Flip> {
        Bound {
            val: self.val,
            inclusion: self.inclusion.flip(),
        }
    }
}

impl<T: FloatCore, B: Boundary> LeftBounded<NotNan<T>, B> {
    pub fn closure(self) -> LeftBounded<NotNan<T>, Inclusive> {
        Bound {
            val: self.val,
            inclusion: Inclusive,
        }
        .into()
    }
    pub fn interior(self) -> LeftBounded<NotNan<T>, Exclusive> {
        Bound {
            val: self.val,
            inclusion: Exclusive,
        }
        .into()
    }
}
impl<T: FloatCore, B: Boundary> RightBounded<NotNan<T>, B> {
    pub fn closure(self) -> RightBounded<NotNan<T>, Inclusive> {
        Bound {
            val: self.val,
            inclusion: Inclusive,
        }
        .into()
    }
    pub fn interior(self) -> RightBounded<NotNan<T>, Exclusive> {
        Bound {
            val: self.val,
            inclusion: Exclusive,
        }
        .into()
    }
}

impl<T: Ord, B: Boundary> LeftBounded<T, B>
where
    Self: Ord,
{
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
}
impl<T: Ord, B: Boundary> RightBounded<T, B>
where
    Self: Ord,
{
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
}

impl<T: FloatCore, B: Boundary> LeftBounded<NotNan<T>, B> {
    pub fn inf(&self) -> NotNan<T> {
        self.val
    }
}
impl<T: FloatCore, B: Boundary> RightBounded<NotNan<T>, B> {
    pub fn sup(&self) -> NotNan<T> {
        self.val
    }
}

impl<T: Clone> Minimum<T> for LeftBounded<T, Inclusive> {
    fn minimum(&self) -> T {
        self.val.clone()
    }
}
impl<T: Clone> Maximum<T> for RightBounded<T, Inclusive> {
    fn maximum(&self) -> T {
        self.val.clone()
    }
}

impl<T: num::Integer + Clone> Minimum<T> for LeftBounded<T, Exclusive> {
    fn minimum(&self) -> T {
        self.val.clone() + T::one()
    }
}
impl<T: num::Integer + Clone> Maximum<T> for RightBounded<T, Exclusive> {
    fn maximum(&self) -> T {
        self.val.clone() - T::one()
    }
}

impl<T: num::Integer + Clone> Minimum<T> for LeftBounded<T, Inclusion> {
    fn minimum(&self) -> T {
        match self.inclusion.0 {
            Inclusion::Inclusive => self.val.clone(),
            Inclusion::Exclusive => self.val.clone() + T::one(),
        }
    }
}
impl<T: num::Integer + Clone> Maximum<T> for RightBounded<T, Inclusion> {
    fn maximum(&self) -> T {
        match self.inclusion.0 {
            Inclusion::Inclusive => self.val.clone(),
            Inclusion::Exclusive => self.val.clone() - T::one(),
        }
    }
}
