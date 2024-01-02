use ordered_float::{FloatCore, NotNan};

use crate::boundary::Boundary;
use crate::{Bound, IntervalIsEmpty, IntoNotNanBound, Lower, Upper};

impl<T: Ord, B: Boundary> Lower<T, B> {
    pub fn includes(&self, other: &Self) -> bool {
        self.val <= other.val
    }
    pub fn contains(&self, t: &T) -> bool {
        self.bound.less(&self.val, t)
    }
}
impl<T: Ord + Clone, B: Boundary> Lower<T, B> {
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

impl<T: Ord, B: Boundary> Upper<T, B> {
    pub fn includes(&self, other: &Self) -> bool {
        other.val <= self.val
    }
    pub fn contains(&self, t: &T) -> bool {
        self.bound.less(t, &self.val)
    }
}
impl<T: Ord + Clone, B: Boundary> Upper<T, B> {
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

pub type UnionSubtrahend<T, L, U> = Interval<T, <U as Boundary>::Flip, <L as Boundary>::Flip>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T, L = crate::Bound, U = L> {
    lower: Lower<T, L>,
    upper: Upper<T, U>,
}
impl<T: Ord + Clone, L: Boundary, U: Boundary> Interval<T, L, U> {
    pub fn new(
        lower: impl Into<Lower<T, L>>,
        upper: impl Into<Upper<T, U>>,
    ) -> Result<Self, IntervalIsEmpty> {
        let lower = lower.into();
        let upper = upper.into();
        (lower.contains(&upper.val) && upper.contains(&lower.val))
            .then_some(Self { lower, upper })
            .ok_or(IntervalIsEmpty)
    }
    pub fn lower(&self) -> &Lower<T, L> {
        &self.lower
    }
    pub fn upper(&self) -> &Upper<T, U> {
        &self.upper
    }

    pub fn contains(&self, t: &T) -> bool {
        self.lower.contains(t) && self.upper.contains(t)
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.lower.includes(&other.lower) && self.upper.includes(&other.upper)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        Self::new(
            self.lower.intersection(&other.lower),
            self.upper.intersection(&other.upper),
        )
        .ok()
    }

    pub fn union_interval(&self, other: &Self) -> Self {
        Self {
            lower: self.lower.union(&other.lower),
            upper: self.upper.union(&other.upper),
        }
    }

    pub fn union_subtrahend(&self, other: &Self) -> Option<Interval<T, U::Flip, L::Flip>> {
        Interval::new(self.upper.flip(), other.lower.flip())
            .or(Interval::new(other.upper.flip(), self.lower.flip()))
            .ok()
    }

    #[allow(clippy::type_complexity)]
    pub fn union(&self, other: &Self) -> (Self, Option<Interval<T, U::Flip, L::Flip>>) {
        (self.union_interval(other), self.union_subtrahend(other))
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }

    pub fn bound<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.union_interval(&item.into())))
    }
}
impl<T: FloatCore, L: Boundary, U: Boundary> Interval<NotNan<T>, L, U> {
    pub fn not_nan(
        lower: impl IntoNotNanBound<L, Float = T>,
        upper: impl IntoNotNanBound<U, Float = T>,
    ) -> Result<Self, crate::Error> {
        let lower = lower.into_not_nan_boundary()?;
        let upper = upper.into_not_nan_boundary()?;
        Self::new(lower, upper).map_err(Into::into)
    }
    pub fn measure(&self) -> NotNan<T> {
        self.upper.val - self.lower.val
    }
    pub fn center(&self) -> NotNan<T> {
        NotNan::new((*self.lower.val + *self.upper.val) / (T::one() + T::one())).unwrap()
    }
}
impl<T> Interval<T> {
    pub fn convert_from<L, U>(src: Interval<T, L, U>) -> Self
    where
        Lower<T, L>: Into<Lower<T, Bound>>,
        Upper<T, U>: Into<Upper<T, Bound>>,
    {
        Self {
            lower: src.lower.into(),
            upper: src.upper.into(),
        }
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
        let _i = Interval::not_nan((1.23, Inclusive), (4.56, Exclusive)).unwrap();

        let i = Interval::bound([3, 9, 2, 5]).unwrap();
        assert_eq!(i.lower().val, 2);
        assert_eq!(i.upper().val, 9);
    }
}
