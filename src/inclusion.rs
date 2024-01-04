use std::marker::PhantomData;

use crate::traits::{Boundary, BoundarySide, Flip, IntoGeneral};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Inclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Exclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Inclusion {
    Inclusive,
    Exclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Left;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Right;

pub trait Side<B> {
    type Ordered: Ord;
    fn make_ordered_inclusion(inclusion: B) -> Self::Ordered;
}

impl<B> Side<B> for Left
where
    SideInclusion<B, Self>: Ord,
{
    type Ordered = SideInclusion<B, Self>;
    fn make_ordered_inclusion(inclusion: B) -> Self::Ordered {
        SideInclusion(inclusion, PhantomData::<Self>)
    }
}
impl<B> Side<B> for Right
where
    SideInclusion<B, Self>: Ord,
{
    type Ordered = SideInclusion<B, Self>;
    fn make_ordered_inclusion(inclusion: B) -> Self::Ordered {
        SideInclusion(inclusion, PhantomData::<Self>)
    }
}

impl<LR> BoundarySide<LR> for Inclusive
where
    SideInclusion<Self, LR>: Ord,
{
    type Ordered = SideInclusion<Self, LR>;
    fn into_ordered(self) -> Self::Ordered {
        SideInclusion(self, PhantomData)
    }
}
impl<LR> BoundarySide<LR> for Exclusive
where
    SideInclusion<Self, LR>: Ord,
{
    type Ordered = SideInclusion<Self, LR>;
    fn into_ordered(self) -> Self::Ordered {
        SideInclusion(self, PhantomData)
    }
}
impl<LR> BoundarySide<LR> for Inclusion
where
    SideInclusion<Self, LR>: Ord,
{
    type Ordered = SideInclusion<Self, LR>;
    fn into_ordered(self) -> Self::Ordered {
        SideInclusion(self, PhantomData)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SideInclusion<B, S>(B, PhantomData<S>);

pub type LeftInclusion<B> = SideInclusion<B, Left>;
pub type RightInclusion<B> = SideInclusion<B, Right>;

mod ordering {
    use super::{LeftInclusion, RightInclusion};
    use crate::{Exclusive, Inclusion, Inclusive};

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
}

impl IntoGeneral for Inclusive {
    type General = Inclusion;
    fn into_general(self) -> Self::General {
        Inclusion::Inclusive
    }
}
impl IntoGeneral for Exclusive {
    type General = Inclusion;
    fn into_general(self) -> Self::General {
        Inclusion::Exclusive
    }
}

impl Flip for Inclusive {
    type Flip = Exclusive;
    fn flip(self) -> Self::Flip {
        Exclusive
    }
}
impl Flip for Exclusive {
    type Flip = Inclusive;
    fn flip(self) -> Self::Flip {
        Inclusive
    }
}
impl Flip for Inclusion {
    type Flip = Self;
    fn flip(self) -> Self {
        match self {
            Self::Inclusive => Self::Exclusive,
            Self::Exclusive => Self::Inclusive,
        }
    }
}

impl Boundary for Inclusive {
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool {
        this <= t
    }
}
impl Boundary for Exclusive {
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool {
        this < t
    }
}
impl Boundary for Inclusion {
    fn less<T: Ord>(&self, s: &T, t: &T) -> bool {
        match self {
            Inclusion::Inclusive => s <= t,
            Inclusion::Exclusive => s < t,
        }
    }
}
