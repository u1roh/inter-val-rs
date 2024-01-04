pub trait Boundary: Eq + Copy {
    type Flip: Boundary<Flip = Self>;
    fn flip(self) -> Self::Flip;
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool;
}

pub trait Minimum<T> {
    fn minimum(&self) -> T;
}

pub trait Maximum<T> {
    fn maximum(&self) -> T;
}
