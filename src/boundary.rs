use crate::{Bound, Exclusive, Inclusive};

pub trait Boundary: Ord + Copy {
    type Flip: Boundary<Flip = Self>;
    fn flip(self) -> Self::Flip;
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool;
}

impl Boundary for Inclusive {
    type Flip = Exclusive;
    fn flip(self) -> Self::Flip {
        Exclusive
    }
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool {
        this <= t
    }
}
impl Boundary for Exclusive {
    type Flip = Inclusive;
    fn flip(self) -> Self::Flip {
        Inclusive
    }
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool {
        this < t
    }
}
impl Boundary for Bound {
    type Flip = Self;
    fn flip(self) -> Self {
        match self {
            Self::Inclusive => Self::Exclusive,
            Self::Exclusive => Self::Inclusive,
        }
    }
    fn less<T: Ord>(&self, s: &T, t: &T) -> bool {
        match self {
            Bound::Inclusive => s <= t,
            Bound::Exclusive => s < t,
        }
    }
}
