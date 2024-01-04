use ordered_float::{FloatCore, NotNan};

use crate::{
    inclusion::{Left, Right},
    traits::{Boundary, IntoGeneral, Maximum, Minimum},
    Bound, Exclusive, Inclusion, Inclusive,
};

#[derive(Debug, Clone, Copy)]
pub struct HalfBounded<T, B, LR>(Bound<T, B>, std::marker::PhantomData<LR>);

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
    use crate::traits::BoundarySide;

    use super::HalfBounded;

    impl<T: Eq, B: Eq, LR> PartialEq for HalfBounded<T, B, LR> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<T: Eq, B: Eq, LR> Eq for HalfBounded<T, B, LR> {}

    impl<T: Ord, B: BoundarySide<LR>, LR> HalfBounded<T, B, LR> {
        fn ordering_key(&self) -> (&T, B::Ordered) {
            (&self.val, self.inclusion.into_ordered())
        }
    }
    impl<T: Ord, B: BoundarySide<LR>, LR> PartialOrd for HalfBounded<T, B, LR> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<T: Ord, B: BoundarySide<LR>, LR> Ord for HalfBounded<T, B, LR> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.ordering_key().cmp(&other.ordering_key())
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
    pub fn flip(self) -> RightBounded<T, B::Flip> {
        HalfBounded(
            Bound {
                val: self.0.val,
                inclusion: self.0.inclusion.flip(),
            },
            std::marker::PhantomData,
        )
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
    pub fn flip(self) -> LeftBounded<T, B::Flip> {
        HalfBounded(
            Bound {
                val: self.0.val,
                inclusion: self.0.inclusion.flip(),
            },
            std::marker::PhantomData,
        )
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
        match self.inclusion {
            Inclusion::Inclusive => self.val.clone(),
            Inclusion::Exclusive => self.val.clone() + T::one(),
        }
    }
}
impl<T: num::Integer + Clone> Maximum<T> for RightBounded<T, Inclusion> {
    fn maximum(&self) -> T {
        match self.inclusion {
            Inclusion::Inclusive => self.val.clone(),
            Inclusion::Exclusive => self.val.clone() - T::one(),
        }
    }
}

impl<T: FloatCore, B: Boundary> LeftBounded<NotNan<T>, B> {
    pub fn inf(&self) -> NotNan<T> {
        self.val
    }
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
    pub fn sup(&self) -> NotNan<T> {
        self.val
    }
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
