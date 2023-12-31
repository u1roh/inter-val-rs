#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inclusive<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Exclusive<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lower<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Upper<T>(pub T);

pub trait Boundary: Ord + Copy {
    type Val: Ord + Copy;
    type Opposite: Boundary<Val = Self::Val, Opposite = Self>;
    fn val(&self) -> Self::Val;
    fn opposite(&self) -> Self::Opposite;
}
impl<T: Copy + Ord> Boundary for Inclusive<T> {
    type Val = T;
    type Opposite = Exclusive<T>;
    fn val(&self) -> T {
        self.0
    }
    fn opposite(&self) -> Self::Opposite {
        Exclusive(self.0)
    }
}
impl<T: Copy + Ord> Boundary for Exclusive<T> {
    type Val = T;
    type Opposite = Inclusive<T>;
    fn val(&self) -> T {
        self.0
    }
    fn opposite(&self) -> Self::Opposite {
        Inclusive(self.0)
    }
}

impl<B: Boundary> Lower<B> {
    fn inf(&self) -> B::Val {
        self.0.val()
    }
    fn includes(&self, other: &Self) -> bool {
        self.inf() <= other.inf()
    }
    fn intersection(&self, other: &Self) -> Self {
        Self(self.0.max(other.0))
    }
    fn union(&self, other: &Self) -> Self {
        Self(self.0.min(other.0))
    }
    pub fn complement(&self) -> Upper<B::Opposite> {
        Upper(self.0.opposite())
    }
}

impl<B: Boundary> Upper<B> {
    fn sup(&self) -> B::Val {
        self.0.val()
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
    pub fn complement(&self) -> Lower<B::Opposite> {
        Lower(self.0.opposite())
    }
}

pub trait Contains<T> {
    fn contains(&self, t: T) -> bool;
}
impl<T: Copy + Ord> Contains<T> for Lower<Inclusive<T>> {
    fn contains(&self, t: T) -> bool {
        self.inf() <= t
    }
}
impl<T: Copy + Ord> Contains<T> for Lower<Exclusive<T>> {
    fn contains(&self, t: T) -> bool {
        self.inf() < t
    }
}
impl<T: Copy + Ord> Contains<T> for Upper<Inclusive<T>> {
    fn contains(&self, t: T) -> bool {
        t <= self.sup()
    }
}
impl<T: Copy + Ord> Contains<T> for Upper<Exclusive<T>> {
    fn contains(&self, t: T) -> bool {
        t < self.sup()
    }
}

pub type UnionSubtrahend<L, U> = Interval<<U as Boundary>::Opposite, <L as Boundary>::Opposite>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<L, U> {
    lower: Lower<L>,
    upper: Upper<U>,
}
impl<L: Boundary, U: Boundary<Val = L::Val>> Interval<L, U>
where
    Lower<L>: Contains<L::Val>,
    Upper<U>: Contains<L::Val>,
{
    fn new_internal(lower: Lower<L>, upper: Upper<U>) -> Option<Self> {
        (lower.contains(upper.sup()) && upper.contains(lower.inf()))
            .then_some(Self { lower, upper })
    }
    pub fn new(lower: L, upper: U) -> Option<Self> {
        Self::new_internal(Lower(lower), Upper(upper))
    }
    pub fn lower(&self) -> &Lower<L> {
        &self.lower
    }
    pub fn upper(&self) -> &Upper<U> {
        &self.upper
    }
    pub fn inf(&self) -> L::Val {
        self.lower.inf()
    }
    pub fn sup(&self) -> U::Val {
        self.upper.sup()
    }

    pub fn measure(&self) -> L::Val
    where
        L::Val: std::ops::Sub<Output = L::Val>,
    {
        self.sup() - self.inf()
    }

    pub fn contains(&self, t: L::Val) -> bool {
        self.lower.contains(t) && self.upper.contains(t)
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.lower.includes(&other.lower) && self.upper.includes(&other.upper)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        Self::new_internal(
            self.lower.intersection(&other.lower),
            self.upper.intersection(&other.upper),
        )
    }

    pub fn union_interval(&self, other: &Self) -> Self {
        Self {
            lower: self.lower.union(&other.lower),
            upper: self.upper.union(&other.upper),
        }
    }

    pub fn union(&self, other: &Self) -> (Self, Option<UnionSubtrahend<L, U>>)
    where
        Lower<U::Opposite>: Contains<L::Val>,
        Upper<L::Opposite>: Contains<L::Val>,
    {
        let subtrahend = Interval::new(self.upper.0.opposite(), other.lower.0.opposite()).or(
            Interval::new(other.upper.0.opposite(), self.lower.0.opposite()),
        );
        (self.union_interval(other), subtrahend)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }
}

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
