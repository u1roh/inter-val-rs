use std::marker::PhantomData;

pub mod general;

pub trait Boundary: std::ops::Deref<Target = Self::Scalar> {
    type Scalar: Copy + Ord;
    fn val(&self) -> Self::Scalar;
    fn contains(&self, t: Self::Scalar) -> bool;
    fn includes(&self, other: &Self) -> bool;
    fn intersection(&self, other: &Self) -> Self;
    fn union(&self, other: &Self) -> Self;
}

pub trait LowerBoundary: Boundary {
    type Complement: UpperBoundary<Scalar = Self::Scalar>;
    fn complement(&self) -> Self::Complement;
}

pub trait UpperBoundary: Boundary {
    type Complement: LowerBoundary<Scalar = Self::Scalar>;
    fn complement(&self) -> Self::Complement;
}

pub struct Inclusive;
pub struct Exclusive;

pub struct Lower<IE>(PhantomData<IE>);
pub struct Upper<IE>(PhantomData<IE>);

trait BoundarySetOperation<T> {
    fn intersection(a: T, b: T) -> T;
    fn union(a: T, b: T) -> T;
    fn includes(boundary: T, t: T) -> bool;
}
impl<T: Copy + Ord, IE> BoundarySetOperation<T> for Lower<IE> {
    fn intersection(a: T, b: T) -> T {
        a.max(b)
    }
    fn union(a: T, b: T) -> T {
        a.min(b)
    }
    fn includes(boundary: T, t: T) -> bool {
        boundary <= t
    }
}
impl<T: Copy + Ord, IE> BoundarySetOperation<T> for Upper<IE> {
    fn intersection(a: T, b: T) -> T {
        a.min(b)
    }
    fn union(a: T, b: T) -> T {
        a.max(b)
    }
    fn includes(boundary: T, t: T) -> bool {
        t <= boundary
    }
}

pub trait BoundaryContains<T> {
    fn contains(boundary: T, t: T) -> bool;
}
impl<T: Copy + Ord> BoundaryContains<T> for Lower<Inclusive> {
    fn contains(min: T, t: T) -> bool {
        min <= t
    }
}
impl<T: Copy + Ord> BoundaryContains<T> for Lower<Exclusive> {
    fn contains(inf: T, t: T) -> bool {
        inf < t
    }
}
impl<T: Copy + Ord> BoundaryContains<T> for Upper<Inclusive> {
    fn contains(max: T, t: T) -> bool {
        t <= max
    }
}
impl<T: Copy + Ord> BoundaryContains<T> for Upper<Exclusive> {
    fn contains(sup: T, t: T) -> bool {
        t < sup
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bound<T, LU>(T, PhantomData<LU>);
pub type LowerInclusive<T> = Bound<T, Lower<Inclusive>>;
pub type LowerExclusive<T> = Bound<T, Lower<Exclusive>>;
pub type UpperInclusive<T> = Bound<T, Upper<Inclusive>>;
pub type UpperExclusive<T> = Bound<T, Upper<Exclusive>>;
impl<T, LU> Bound<T, LU> {
    pub fn new(t: T) -> Self {
        Self(t, PhantomData)
    }
}
impl<T, LU> std::ops::Deref for Bound<T, LU> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Copy + Ord, LU: BoundarySetOperation<T> + BoundaryContains<T>> Boundary for Bound<T, LU> {
    type Scalar = T;
    fn val(&self) -> Self::Scalar {
        self.0
    }
    fn contains(&self, t: Self::Scalar) -> bool {
        LU::contains(self.0, t)
    }
    fn includes(&self, other: &Self) -> bool {
        LU::includes(self.0, other.0)
    }
    fn intersection(&self, other: &Self) -> Self {
        Self(LU::intersection(self.0, other.0), PhantomData)
    }
    fn union(&self, other: &Self) -> Self {
        Self(LU::union(self.0, other.0), PhantomData)
    }
}

pub fn inclusive<T>(t: T) -> LowerInclusive<T> {
    LowerInclusive::new(t)
}

pub type UnionSubtrahend<L, U> =
    Interval<<U as UpperBoundary>::Complement, <L as LowerBoundary>::Complement>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<L, U> {
    pair: (L, U),
}
impl<L, U> std::ops::Deref for Interval<L, U> {
    type Target = (L, U);
    fn deref(&self) -> &Self::Target {
        &self.pair
    }
}
impl<L, U> Interval<L, U>
where
    L: LowerBoundary,
    U: UpperBoundary<Scalar = L::Scalar>,
{
    pub fn new(lower: L, upper: U) -> Option<Self> {
        (lower.contains(upper.val()) && upper.contains(lower.val())).then_some(Self {
            pair: (lower, upper),
        })
    }
    pub fn inf(&self) -> L::Scalar {
        self.0.val()
    }
    pub fn sup(&self) -> U::Scalar {
        self.1.val()
    }

    pub fn measure(&self) -> L::Scalar
    where
        L::Scalar: std::ops::Sub<Output = L::Scalar>,
    {
        self.sup() - self.inf()
    }

    pub fn contains(&self, t: L::Scalar) -> bool {
        self.0.contains(t) && self.1.contains(t)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        Self::new(self.0.intersection(&other.0), self.1.intersection(&other.1))
    }

    pub fn union_interval(&self, other: &Self) -> Self {
        Self {
            pair: (self.0.union(&other.0), self.1.union(&other.1)),
        }
    }

    pub fn union(&self, other: &Self) -> (Self, Option<UnionSubtrahend<L, U>>) {
        let subtrahend = Interval::new(self.1.complement(), other.0.complement())
            .or(Interval::new(other.1.complement(), self.0.complement()));
        (self.union_interval(other), subtrahend)
    }

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
    struct Piyo;

    #[test]
    fn it_works() {
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
