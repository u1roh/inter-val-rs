use crate::boundary::Boundary;
use crate::{IntervalIsEmpty, Lower, Upper};

impl<B: Boundary> Lower<B> {
    pub fn inf(&self) -> &B::Val {
        self.0.val()
    }
    pub fn includes(&self, other: &Self) -> bool {
        self.inf() <= other.inf()
    }
    pub fn contains(&self, t: &B::Val) -> bool {
        self.0.less_eq(t)
    }
}
impl<B: Boundary + Clone> Lower<B> {
    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0.clone().max(other.0.clone()))
    }
    pub fn union(&self, other: &Self) -> Self {
        Self(self.0.clone().min(other.0.clone()))
    }
    pub fn flip(&self) -> Upper<B::Flip> {
        Upper(self.0.clone().flip())
    }
}

impl<B: Boundary> Upper<B> {
    pub fn sup(&self) -> &B::Val {
        self.0.val()
    }
    pub fn includes(&self, other: &Self) -> bool {
        other.0 <= self.0
    }
    pub fn contains(&self, t: &B::Val) -> bool {
        self.0.greater_eq(t)
    }
}
impl<B: Boundary + Clone> Upper<B> {
    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0.clone().min(other.0.clone()))
    }
    pub fn union(&self, other: &Self) -> Self {
        Self(self.0.clone().max(other.0.clone()))
    }
    pub fn flip(&self) -> Lower<B::Flip> {
        Lower(self.0.clone().flip())
    }
}

pub type UnionSubtrahend<L, U> = Interval<<U as Boundary>::Flip, <L as Boundary>::Flip>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<L, U = L> {
    lower: Lower<L>,
    upper: Upper<U>,
}
impl<L: Boundary, U: Boundary<Val = L::Val>> Interval<L, U> {
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
    {
        let subtrahend = Interval::new_(self.upper.flip(), other.lower.flip())
            .or(Interval::new_(other.upper.flip(), self.lower.flip()))
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
    use crate::{Exclusive, Inclusive};

    #[test]
    fn it_works() {
        let i = Interval::new(Inclusive(0), Exclusive(3)).unwrap();
        assert!(i.contains(&0));
        assert!(i.contains(&1));
        assert!(i.contains(&2));
        assert!(!i.contains(&3));
        assert!(!i.contains(&-1));

        let i = Inclusive(4).to(Inclusive(7)).unwrap();
        assert!(i.contains(&4));
        assert!(i.contains(&7));

        let i = Exclusive(-2).to(Inclusive(5)).unwrap();
        assert!(!i.contains(&-2));
        assert!(i.contains(&5));

        let _i = Interval::new(
            Inclusive::not_nan(1.23).unwrap(),
            Exclusive::not_nan(4.56).unwrap(),
        )
        .unwrap();
        let _i = Inclusive::not_nan(1.23)
            .unwrap()
            .to(Exclusive::not_nan(4.56).unwrap())
            .unwrap();
    }
}
