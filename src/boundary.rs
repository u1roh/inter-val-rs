use crate::{Exclusive, Inclusive};

pub trait Boundary: Ord {
    type Val: Ord;
    type Flip: Boundary<Val = Self::Val, Flip = Self>;
    fn val(&self) -> &Self::Val;
    fn flip(self) -> Self::Flip;
    fn less_eq(&self, t: &Self::Val) -> bool;
    fn greater_eq(&self, t: &Self::Val) -> bool;
}
impl<T: Ord> Boundary for Inclusive<T> {
    type Val = T;
    type Flip = Exclusive<T>;
    fn val(&self) -> &T {
        &self.0
    }
    fn flip(self) -> Self::Flip {
        Exclusive(self.0)
    }
    fn less_eq(&self, t: &Self::Val) -> bool {
        self.val() <= t
    }
    fn greater_eq(&self, t: &Self::Val) -> bool {
        t <= self.val()
    }
}
impl<T: Ord> Boundary for Exclusive<T> {
    type Val = T;
    type Flip = Inclusive<T>;
    fn val(&self) -> &T {
        &self.0
    }
    fn flip(self) -> Self::Flip {
        Inclusive(self.0)
    }
    fn less_eq(&self, t: &Self::Val) -> bool {
        self.val() < t
    }
    fn greater_eq(&self, t: &Self::Val) -> bool {
        t < self.val()
    }
}
impl<T: Ord> Boundary for crate::Boundary<T> {
    type Val = T;
    type Flip = Self;
    fn val(&self) -> &T {
        match self {
            Self::Inclusive(t) => t,
            Self::Exclusive(t) => t,
        }
    }
    fn flip(self) -> Self {
        match self {
            Self::Inclusive(t) => Self::Exclusive(t),
            Self::Exclusive(t) => Self::Inclusive(t),
        }
    }
    fn less_eq(&self, t: &Self::Val) -> bool {
        match self {
            Self::Inclusive(s) => s <= t,
            Self::Exclusive(s) => s < t,
        }
    }
    fn greater_eq(&self, t: &Self::Val) -> bool {
        match self {
            Self::Inclusive(s) => t <= s,
            Self::Exclusive(s) => t < s,
        }
    }
}
