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

trait Boundary<T> {
    fn is_valid(&self, lower: T, upper: T) -> bool;
    fn are_inclusive(&self) -> (bool, bool);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosedInterval<T> {
    pair: (T, T),
}
impl<T> std::ops::Deref for ClosedInterval<T> {
    type Target = (T, T);
    fn deref(&self) -> &Self::Target {
        &self.pair
    }
}
impl<T: Copy + Ord> ClosedInterval<T> {
    pub fn new(lower: T, upper: T) -> Option<Self> {
        (lower <= upper).then_some(Self {
            pair: (lower, upper),
        })
    }
    pub fn contains(&self, t: T) -> bool {
        self.0 <= t && t <= self.1 // closed interval
    }
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        Self::new(self.0.max(other.0), self.1.min(other.1))
    }
    pub fn union(&self, other: &Self) -> (Self, Option<Self>) {
        if self.1 < other.0 {
            (*self, Some(*other))
        } else if other.1 < self.0 {
            (*other, Some(*self))
        } else {
            let pair = (self.0.min(other.0), self.1.max(other.1));
            (Self { pair }, None)
        }
    }
    pub fn overlaps(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }
    pub fn includes(&self, other: &Self) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }
    pub fn signed_distance(&self, other: Self) -> T
    where
        T: std::ops::Sub,
    {
    }
}

pub trait Interval<T>: Sized + std::ops::Deref<Target = (T, T)> {
    fn lower(&self) -> T;
    fn upper(&self) -> T;
    fn is_inclusive(&self) -> (bool, bool);
    fn measure(&self) -> T;
    fn contains(&self, t: T) -> bool;
    fn intersection(&self, other: &Self) -> Option<Self>;
    fn union(&self, other: &Self) -> (Self, Option<Self>);
}

pub trait IntervalSet<T>: std::ops::Deref<Target = [Self::Interval]> {
    type Interval: Interval<T>;
    type Complement: IntervalSet<T>;
    type Difference: IntervalSet<T>;
    fn intersection(&self, other: &Self) -> Self;
    fn union(&self, other: &Self) -> Self;
    fn complement(&self) -> Self::Complement;
    fn measure(&self) -> T;
    fn overlaps(&self, other: &Self) -> bool;
}

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
