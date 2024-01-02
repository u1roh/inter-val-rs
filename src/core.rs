use ordered_float::{FloatCore, NotNan};

use crate::boundary::Boundary;
use crate::{IntervalIsEmpty, Lower, Upper};

impl<T: Ord, B: Boundary<T>> Lower<T, B> {
    pub fn inf(&self) -> &T {
        &self.val
    }
    pub fn includes(&self, other: &Self) -> bool {
        self.val <= other.val
    }
    pub fn contains(&self, t: &T) -> bool {
        self.bound.less(&self.val, t)
    }
}
impl<T: Ord + Clone, B: Boundary<T>> Lower<T, B> {
    pub fn intersection(&self, other: &Self) -> Self {
        self.clone().max(other.clone())
    }
    pub fn union(&self, other: &Self) -> Self {
        self.clone().min(other.clone())
    }
    pub fn flip(&self) -> Upper<T, B::Flip> {
        Upper {
            val: self.val.clone(),
            bound: self.bound.flip(),
        }
    }
}

impl<T: Ord, B: Boundary<T>> Upper<T, B> {
    pub fn sup(&self) -> &T {
        &self.val
    }
    pub fn includes(&self, other: &Self) -> bool {
        other.val <= self.val
    }
    pub fn contains(&self, t: &T) -> bool {
        self.bound.less(t, &self.val)
    }
}
impl<T: Ord + Clone, B: Boundary<T>> Upper<T, B> {
    pub fn intersection(&self, other: &Self) -> Self {
        self.clone().min(other.clone())
    }
    pub fn union(&self, other: &Self) -> Self {
        self.clone().max(other.clone())
    }
    pub fn flip(&self) -> Lower<T, B::Flip> {
        Lower {
            val: self.val.clone(),
            bound: self.bound.flip(),
        }
    }
}

pub type UnionSubtrahend<T, L, U> = Interval<T, <U as Boundary<T>>::Flip, <L as Boundary<T>>::Flip>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T, L = crate::Bound, U = L> {
    lower: Lower<T, L>,
    upper: Upper<T, U>,
}
impl<T: FloatCore, L: Boundary<NotNan<T>>, U: Boundary<NotNan<T>>> Interval<NotNan<T>, L, U> {
    pub fn not_nan(
        lower: impl Into<Lower<T, L>>,
        upper: impl Into<Upper<T, U>>,
    ) -> Result<Self, crate::Error> {
        let lower = lower.into();
        let upper = upper.into();
        Self::new(
            (NotNan::new(lower.val)?, lower.bound),
            (NotNan::new(upper.val)?, upper.bound),
        )
        .map_err(Into::into)
    }
}
impl<T: Ord + Clone, L: Boundary<T>, U: Boundary<T>> Interval<T, L, U> {
    pub fn new(
        lower: impl Into<Lower<T, L>>,
        upper: impl Into<Upper<T, U>>,
    ) -> Result<Self, IntervalIsEmpty> {
        let lower = lower.into();
        let upper = upper.into();
        (lower.contains(upper.sup()) && upper.contains(lower.inf()))
            .then_some(Self { lower, upper })
            .ok_or(IntervalIsEmpty)
    }
    // pub fn new(lower: (T, L), upper: (T, U)) -> Result<Self, IntervalIsEmpty> {
    //     Self::new_(
    //         Lower {
    //             val: lower.0,
    //             bound: lower.1,
    //         },
    //         Upper {
    //             val: upper.0,
    //             bound: upper.1,
    //         },
    //     )
    // }
    pub fn lower(&self) -> &Lower<T, L> {
        &self.lower
    }
    pub fn upper(&self) -> &Upper<T, U> {
        &self.upper
    }
    pub fn inf(&self) -> &T {
        self.lower.inf()
    }
    pub fn sup(&self) -> &T {
        self.upper.sup()
    }

    pub fn measure(&self) -> T
    where
        for<'a> &'a T: std::ops::Sub<Output = T>,
    {
        self.sup() - self.inf()
    }

    pub fn contains(&self, t: &T) -> bool {
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
        Self::new(
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

    pub fn union(&self, other: &Self) -> (Self, Option<UnionSubtrahend<T, L, U>>)
    where
        L: Clone,
        U: Clone,
    {
        let subtrahend = Interval::new(self.upper.flip(), other.lower.flip())
            .or(Interval::new(other.upper.flip(), self.lower.flip()))
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
        let i = Interval::new((0, Inclusive), (3, Exclusive)).unwrap();
        assert!(i.contains(&0));
        assert!(i.contains(&1));
        assert!(i.contains(&2));
        assert!(!i.contains(&3));
        assert!(!i.contains(&-1));

        // let i = Inclusive(4).to(Inclusive(7)).unwrap();
        // assert!(i.contains(&4));
        // assert!(i.contains(&7));

        // let i = Exclusive(-2).to(Inclusive(5)).unwrap();
        // assert!(!i.contains(&-2));
        // assert!(i.contains(&5));

        let _i = Interval::<NotNan<_>, Inclusive, Inclusive>::not_nan(1.23, 4.56).unwrap();
        // let _i = Interval::<NotNan<_>, Inclusive, Inclusive>::not_nan(
        //     (1.23, Inclusive),
        //     (4.56, Exclusive),
        // )
        // .unwrap();
        // let _i = Inclusive::not_nan(1.23)
        //     .unwrap()
        //     .to(Exclusive::not_nan(4.56).unwrap())
        //     .unwrap();
    }
}
