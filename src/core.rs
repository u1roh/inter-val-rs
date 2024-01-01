use crate::{Exclusive, Inclusive, IntervalIsEmpty};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lower<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Upper<T>(pub T);

pub trait Boundary: Ord {
    type Val: Ord;
    type Fellow: Boundary<Val = Self::Val, Fellow = Self>;
    fn val(&self) -> &Self::Val;
    fn fellow(self) -> Self::Fellow;
}
impl<T: Ord> Boundary for Inclusive<T> {
    type Val = T;
    type Fellow = Exclusive<T>;
    fn val(&self) -> &T {
        &self.0
    }
    fn fellow(self) -> Self::Fellow {
        Exclusive(self.0)
    }
}
impl<T: Ord> Boundary for Exclusive<T> {
    type Val = T;
    type Fellow = Inclusive<T>;
    fn val(&self) -> &T {
        &self.0
    }
    fn fellow(self) -> Self::Fellow {
        Inclusive(self.0)
    }
}
impl<T: Ord> Boundary for crate::Boundary<T> {
    type Val = T;
    type Fellow = Self;
    fn val(&self) -> &T {
        match self {
            Self::Inclusive(t) => t,
            Self::Exclusive(t) => t,
        }
    }
    fn fellow(self) -> Self {
        match self {
            Self::Inclusive(t) => Self::Exclusive(t),
            Self::Exclusive(t) => Self::Inclusive(t),
        }
    }
}

impl<B: Boundary> Lower<B> {
    fn inf(&self) -> &B::Val {
        self.0.val()
    }
    fn includes(&self, other: &Self) -> bool {
        self.inf() <= other.inf()
    }
}
impl<B: Boundary + Clone> Lower<B> {
    fn intersection(&self, other: &Self) -> Self {
        Self(self.0.clone().max(other.0.clone()))
    }
    fn union(&self, other: &Self) -> Self {
        Self(self.0.clone().min(other.0.clone()))
    }
    pub fn complement(&self) -> Upper<B::Fellow> {
        Upper(self.0.clone().fellow())
    }
}

impl<B: Boundary> Upper<B> {
    fn sup(&self) -> &B::Val {
        self.0.val()
    }
    fn includes(&self, other: &Self) -> bool {
        other.0 <= self.0
    }
}
impl<B: Boundary + Clone> Upper<B> {
    fn intersection(&self, other: &Self) -> Self {
        Self(self.0.clone().min(other.0.clone()))
    }
    fn union(&self, other: &Self) -> Self {
        Self(self.0.clone().max(other.0.clone()))
    }
    pub fn complement(&self) -> Lower<B::Fellow> {
        Lower(self.0.clone().fellow())
    }
}

pub trait Contains<T> {
    fn contains(&self, t: &T) -> bool;
}
impl<T: Ord> Contains<T> for Lower<Inclusive<T>> {
    fn contains(&self, t: &T) -> bool {
        self.inf() <= t
    }
}
impl<T: Ord> Contains<T> for Lower<Exclusive<T>> {
    fn contains(&self, t: &T) -> bool {
        self.inf() < t
    }
}
impl<T: Ord> Contains<T> for Upper<Inclusive<T>> {
    fn contains(&self, t: &T) -> bool {
        t <= self.sup()
    }
}
impl<T: Ord> Contains<T> for Upper<Exclusive<T>> {
    fn contains(&self, t: &T) -> bool {
        t < self.sup()
    }
}

impl<T: Ord> std::ops::RangeBounds<T> for Lower<Inclusive<T>> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Included(self.inf())
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Unbounded
    }
}
impl<T: Ord> std::ops::RangeBounds<T> for Lower<Exclusive<T>> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Excluded(self.inf())
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Unbounded
    }
}
impl<T: Ord> std::ops::RangeBounds<T> for Upper<Inclusive<T>> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Unbounded
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Included(self.sup())
    }
}
impl<T: Ord> std::ops::RangeBounds<T> for Upper<Exclusive<T>> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Unbounded
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Excluded(self.sup())
    }
}

pub type UnionSubtrahend<L, U> = Interval<<U as Boundary>::Fellow, <L as Boundary>::Fellow>;

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
    fn new_(lower: Lower<L>, upper: Upper<U>) -> Result<Self, IntervalIsEmpty> {
        (lower.contains(upper.sup()) && upper.contains(lower.inf()))
            .then_some(Self { lower, upper })
            .ok_or(IntervalIsEmpty)
    }
    pub fn new(lower: L, upper: U) -> Result<Self, IntervalIsEmpty> {
        Self::new_(Lower(lower), Upper(upper))
    }
    pub fn lower(&self) -> &Lower<L> {
        &self.lower
    }
    pub fn upper(&self) -> &Upper<U> {
        &self.upper
    }
    pub fn inf(&self) -> &L::Val {
        self.lower.inf()
    }
    pub fn sup(&self) -> &U::Val {
        self.upper.sup()
    }

    pub fn measure(&self) -> L::Val
    where
        for<'a> &'a L::Val: std::ops::Sub<Output = L::Val>,
    {
        self.sup() - self.inf()
    }

    pub fn contains(&self, t: &L::Val) -> bool {
        self.lower.contains(t) && self.upper.contains(t)
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.lower.includes(&other.lower) && self.upper.includes(&other.upper)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self>
    where
        L: Clone,
        U: Clone,
    {
        Self::new_(
            self.lower.intersection(&other.lower),
            self.upper.intersection(&other.upper),
        )
        .ok()
    }

    pub fn union_interval(&self, other: &Self) -> Self
    where
        L: Clone,
        U: Clone,
    {
        Self {
            lower: self.lower.union(&other.lower),
            upper: self.upper.union(&other.upper),
        }
    }

    pub fn union(&self, other: &Self) -> (Self, Option<UnionSubtrahend<L, U>>)
    where
        L: Clone,
        U: Clone,
        Lower<U::Fellow>: Contains<L::Val>,
        Upper<L::Fellow>: Contains<L::Val>,
    {
        let subtrahend = Interval::new_(self.upper.complement(), other.lower.complement())
            .or(Interval::new_(
                other.upper.complement(),
                self.lower.complement(),
            ))
            .ok();
        (self.union_interval(other), subtrahend)
    }

    pub fn overlaps(&self, other: &Self) -> bool
    where
        L: Clone,
        U: Clone,
    {
        self.intersection(other).is_some()
    }
}
impl<L: Boundary, U: Boundary<Val = L::Val>> std::ops::RangeBounds<L::Val> for Interval<L, U>
where
    Lower<L>: Contains<L::Val>,
    Upper<U>: Contains<L::Val>,
{
    fn start_bound(&self) -> std::ops::Bound<&L::Val> {
        std::ops::Bound::Included(self.inf())
    }
    fn end_bound(&self) -> std::ops::Bound<&L::Val> {
        std::ops::Bound::Excluded(self.sup())
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
        assert!(i.contains(&0));
        assert!(i.contains(&1));
        assert!(i.contains(&2));
        assert!(!i.contains(&3));
        assert!(!i.contains(&-1));

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
