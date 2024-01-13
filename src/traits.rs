use crate::BoundType;

pub trait Flip {
    type Flip: Flip<Flip = Self>;
    fn flip(self) -> Self::Flip;
}

pub(crate) trait IntoGeneral {
    type General;
    fn into_general(self) -> Self::General;
}

pub trait Boundary: Flip + Eq + PartialEq<BoundType> + Copy {
    fn less<T: PartialOrd>(&self, this: &T, t: &T) -> bool;

    fn is_inclusive(&self) -> bool {
        *self == BoundType::Inclusive
    }
    fn is_exclusive(&self) -> bool {
        *self == BoundType::Exclusive
    }
}

pub trait BoundaryOf<LR>: Boundary {
    type Ordered: Ord;
    fn into_ordered(self) -> Self::Ordered;
}
