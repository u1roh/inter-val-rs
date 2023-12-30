pub mod general;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntervalComparison {
    Equal,
    Less,
    Greater,
    OverlapLess,
    OverlapGreater,
    OverlapSubset,
    OverlapSuperset,
}

trait Hoge {
    fn hoge(&self) -> &str;
}

impl Hoge for std::marker::PhantomData<usize> {
    fn hoge(&self) -> &str {
        "hoge"
    }
}

pub trait Boundary: Copy + std::ops::Deref<Target = Self::Scalar> {
    type Scalar: Copy + Ord;
    fn val(&self) -> Self::Scalar;
    fn is_inclusive(&self) -> bool;
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

struct InclusiveLower<T>(T);
impl<T> std::ops::Deref for InclusiveLower<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: PartialOrd> InclusiveLower<T> {
    pub fn contains(&self, t: T) -> bool {
        self.0 <= t
    }
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
        // (lower.is_less_or_eq(upper.val()) && upper.is_greater_or_eq(lower.val())).then_some(Self {
        //     pair: (lower, upper),
        // })
        // (
        //     *lower < *upper || (*lower == *upper && lower.is_inclusive() && upper.is_inclusive())
        // )
        // .then_some(Self {
        //     pair: (lower, upper),
        // })
    }
    pub fn inf(&self) -> L::Scalar {
        self.0.val()
    }
    pub fn sup(&self) -> U::Scalar {
        self.1.val()
    }
    pub fn contains(&self, t: L::Scalar) -> bool {
        self.0.contains(t) && self.1.contains(t)
        // self.0.is_less_or_eq(t) && self.1.is_greater_or_eq(t)
        // (*self.0 < t || (*self.0 == t && self.0.is_inclusive()))
        //     && (t < *self.1 || (t == *self.1 && self.1.is_inclusive()))
        // self.0 <= t && t <= self.1 // closed interval
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
    use super::*;

    struct Piyo;

    #[test]
    fn it_works() {
        let a = std::marker::PhantomData::<usize>;
        let b = Piyo;
        dbg!(std::mem::size_of::<Piyo>());
        dbg!(std::mem::size_of_val(&b));
        assert_eq!(std::mem::size_of::<std::marker::PhantomData<usize>>(), 0);
        assert_eq!(std::mem::size_of_val(&a), 0);
        assert_eq!(&a as *const _, std::ptr::null());
        assert_eq!(a.hoge(), "hoge");
        assert_eq!(2 + 2, 4);
    }
}
