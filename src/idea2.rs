use std::marker::PhantomData;

pub trait Boundary: std::ops::Deref<Target = Self::Scalar> {
    type Scalar: Copy + Ord;
    fn val(&self) -> Self::Scalar;
    fn includes(&self, other: &Self) -> bool;
    fn intersection(&self, other: &Self) -> Self;
    fn union(&self, other: &Self) -> Self;
}

pub trait Contains<T> {
    fn contains(&self, t: T) -> bool;
}

// pub trait LowerBoundary: Boundary + Contains<Self::Scalar> {
pub trait LowerBoundary: Boundary {
    type Complement: UpperBoundary<Scalar = Self::Scalar>;
    fn complement(&self) -> Self::Complement;
}

// pub trait UpperBoundary: Boundary + Contains<Self::Scalar> {
pub trait UpperBoundary: Boundary {
    type Complement: LowerBoundary<Scalar = Self::Scalar>;
    fn complement(&self) -> Self::Complement;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inclusive<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Exclusive<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lower<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Upper<T>(pub T);

impl<T> std::ops::Deref for Inclusive<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> std::ops::Deref for Exclusive<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Opposite: std::ops::Deref + Ord + Copy {
    type Opposite: Opposite<Opposite = Self> + std::ops::Deref<Target = Self::Target>;
    fn opposite(&self) -> Self::Opposite;
}
impl<T: Copy + Ord> Opposite for Inclusive<T> {
    type Opposite = Exclusive<T>;
    fn opposite(&self) -> Self::Opposite {
        Exclusive(self.0)
    }
}
impl<T: Copy + Ord> Opposite for Exclusive<T> {
    type Opposite = Inclusive<T>;
    fn opposite(&self) -> Self::Opposite {
        Inclusive(self.0)
    }
}

impl<IE: std::ops::Deref> std::ops::Deref for Lower<IE> {
    type Target = IE::Target;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl<IE: Opposite> Boundary for Lower<IE>
where
    IE::Target: Copy + Ord,
{
    type Scalar = IE::Target;
    fn val(&self) -> Self::Scalar {
        *self.0
    }
    fn includes(&self, other: &Self) -> bool {
        self.val() <= other.val()
    }
    fn intersection(&self, other: &Self) -> Self {
        Self(self.0.max(other.0))
    }
    fn union(&self, other: &Self) -> Self {
        Self(self.0.min(other.0))
    }
}

impl<IE: std::ops::Deref> std::ops::Deref for Upper<IE> {
    type Target = IE::Target;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl<IE: Opposite> Boundary for Upper<IE>
where
    IE::Target: Copy + Ord,
{
    type Scalar = IE::Target;
    fn val(&self) -> Self::Scalar {
        *self.0
    }
    fn includes(&self, other: &Self) -> bool {
        other.0 <= self.0
    }
    fn intersection(&self, other: &Self) -> Self {
        Self(self.0.min(other.0))
    }
    fn union(&self, other: &Self) -> Self {
        Self(self.0.max(other.0))
    }
}

impl<T: Copy + Ord> Contains<T> for Lower<Inclusive<T>> {
    fn contains(&self, t: T) -> bool {
        self.val() <= t
    }
}
impl<T: Copy + Ord> Contains<T> for Lower<Exclusive<T>> {
    fn contains(&self, t: T) -> bool {
        self.val() < t
    }
}
impl<T: Copy + Ord> Contains<T> for Upper<Inclusive<T>> {
    fn contains(&self, t: T) -> bool {
        t <= self.val()
    }
}
impl<T: Copy + Ord> Contains<T> for Upper<Exclusive<T>> {
    fn contains(&self, t: T) -> bool {
        t < self.val()
    }
}

impl<IE: Opposite> LowerBoundary for Lower<IE>
where
    IE::Target: Copy + Ord,
    // Self: Contains<IE::Target>,
{
    type Complement = Upper<IE::Opposite>;
    fn complement(&self) -> Self::Complement {
        Upper(self.0.opposite())
    }
}
impl<IE: Opposite> UpperBoundary for Upper<IE>
where
    IE::Target: Copy + Ord,
    // Self: Contains<IE::Target>,
{
    type Complement = Lower<IE::Opposite>;
    fn complement(&self) -> Self::Complement {
        Lower(self.0.opposite())
    }
}

pub type UnionSubtrahend<L, U> =
    Interval<<Upper<U> as UpperBoundary>::Complement, <Lower<L> as LowerBoundary>::Complement>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<L, U> {
    pair: (Lower<L>, Upper<U>),
}
impl<L, U> std::ops::Deref for Interval<L, U> {
    type Target = (Lower<L>, Upper<U>);
    fn deref(&self) -> &Self::Target {
        &self.pair
    }
}
impl<L: Opposite, U: Opposite<Target = L::Target>> Interval<L, U>
where
    L::Target: Sized + Copy + Ord,
    Lower<L>: Contains<L::Target>,
    Upper<U>: Contains<L::Target>,
{
    fn new_internal(lower: Lower<L>, upper: Upper<U>) -> Option<Self> {
        (lower.contains(upper.val()) && upper.contains(lower.val())).then_some(Self {
            pair: (lower, upper),
        })
    }
    pub fn new(lower: L, upper: U) -> Option<Self> {
        Self::new_internal(Lower(lower), Upper(upper))
    }
    pub fn inf(&self) -> L::Target {
        self.0.val()
    }
    pub fn sup(&self) -> U::Target {
        self.1.val()
    }

    pub fn measure(&self) -> L::Target
    where
        L::Target: std::ops::Sub<Output = L::Target>,
    {
        self.sup() - self.inf()
    }

    pub fn contains(&self, t: L::Target) -> bool {
        self.0.contains(t) && self.1.contains(t)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        Self::new_internal(self.0.intersection(&other.0), self.1.intersection(&other.1))
    }

    pub fn union_interval(&self, other: &Self) -> Self {
        Self {
            pair: (self.0.union(&other.0), self.1.union(&other.1)),
        }
    }

    // pub fn union(&self, other: &Self) -> (Self, Option<UnionSubtrahend<L, U>>) {
    //     let subtrahend = Interval::new_internal(self.1.complement(), other.0.complement()).or(
    //         Interval::new_internal(other.1.complement(), self.0.complement()),
    //     );
    //     (self.union_interval(other), subtrahend)
    // }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.0.includes(&other.0) && self.1.includes(&other.1)
    }
}

// pub trait Interval<T>: Sized + std::ops::Deref<Target = (T, T)> {
//     fn lower(&self) -> T;
//     fn upper(&self) -> T;
//     fn is_inclusive(&self) -> (bool, bool);
//     fn measure(&self) -> T;
//     fn contains(&self, t: T) -> bool;
//     fn intersection(&self, other: &Self) -> Option<Self>;
//     fn union(&self, other: &Self) -> (Self, Option<Self>);
// }

// pub trait IntervalSet<T>: std::ops::Deref<Target = [Self::Interval]> {
//     type Interval: Interval<T>;
//     type Complement: IntervalSet<T>;
//     type Difference: IntervalSet<T>;
//     fn intersection(&self, other: &Self) -> Self;
//     fn union(&self, other: &Self) -> Self;
//     fn complement(&self) -> Self::Complement;
//     fn measure(&self) -> T;
//     fn overlaps(&self, other: &Self) -> bool;
// }

#[cfg(test)]
mod tests {
    use super::*;

    struct Piyo;

    #[test]
    fn it_works() {
        let i = Interval::new(Inclusive(0), Exclusive(3)).unwrap();
        assert!(i.contains(0));
        assert!(i.contains(1));
        assert!(i.contains(2));
        assert!(!i.contains(3));
        assert!(!i.contains(-1));

        let a = std::marker::PhantomData::<usize>;
        let b = Piyo;
        dbg!(std::mem::size_of::<Piyo>());
        dbg!(std::mem::size_of_val(&b));
        dbg!(std::mem::size_of::<std::marker::PhantomData<usize>>());
        dbg!(std::mem::size_of_val(&a));
        // assert_eq!(&a as *const _, std::ptr::null());
        assert_eq!(2 + 2, 4);
    }
}
