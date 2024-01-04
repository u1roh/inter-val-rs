pub trait Flip {
    type Flip: Flip<Flip = Self>;
    fn flip(self) -> Self::Flip;
}

pub trait Boundary: Flip + Eq + Copy {
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool;
}

pub trait BoundarySide<LR>: Boundary {
    type Ordered: Ord;
    fn into_ordered(self) -> Self::Ordered;
}

pub trait Minimum<T> {
    fn minimum(&self) -> T;
}

pub trait Maximum<T> {
    fn maximum(&self) -> T;
}

pub(crate) trait IntoGeneral {
    type General;
    fn into_general(self) -> Self::General;
}
