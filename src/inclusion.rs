use crate::traits::{Boundary, Flip, IntoGeneral};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Inclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Exclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Inclusion {
    Inclusive,
    Exclusive,
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
