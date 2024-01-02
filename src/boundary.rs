use crate::{Bound, Exclusive, Inclusive};

pub trait Boundary<T>: Ord + Copy {
    type Flip: Boundary<T, Flip = Self>;
    fn flip(self) -> Self::Flip;
    fn less(&self, this: &T, t: &T) -> bool;
}

impl<T: Ord> Boundary<T> for Inclusive {
    type Flip = Exclusive;
    fn flip(self) -> Self::Flip {
        Exclusive
    }
    fn less(&self, this: &T, t: &T) -> bool {
        this <= t
    }
}
impl<T: Ord> Boundary<T> for Exclusive {
    type Flip = Inclusive;
    fn flip(self) -> Self::Flip {
        Inclusive
    }
    fn less(&self, this: &T, t: &T) -> bool {
        this < t
    }
}
impl<T: Ord> Boundary<T> for Bound {
    type Flip = Self;
    fn flip(self) -> Self {
        match self {
            Self::Inclusive => Self::Exclusive,
            Self::Exclusive => Self::Inclusive,
        }
    }
    fn less(&self, s: &T, t: &T) -> bool {
        match self {
            Bound::Inclusive => s <= t,
            Bound::Exclusive => s < t,
        }
    }
}
