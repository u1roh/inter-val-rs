use crate::BoundType;

pub trait Flip {
    type Flip: Flip<Flip = Self>;
    fn flip(self) -> Self::Flip;
}

pub trait Ceil<T> {
    fn ceil(&self) -> T;
}

pub trait Floor<T> {
    fn floor(&self) -> T;
}

pub(crate) trait IntoGeneral {
    type General;
    fn into_general(self) -> Self::General;
}

pub trait Boundary: Flip + Eq + PartialEq<BoundType> + Copy {
    fn less<T: PartialOrd>(&self, this: &T, t: &T) -> bool;
}

pub trait BoundaryOf<LR>: Boundary {
    type Ordered: Ord;
    fn into_ordered(self) -> Self::Ordered;
}
