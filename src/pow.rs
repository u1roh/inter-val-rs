use ordered_float::{FloatCore, NotNan};

use crate::{boundary::Boundary, Bound, Exclusive, Inclusive, Interval, Lower, Upper};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalPow<const N: usize, T, L = Bound, U = L>([Interval<T, L, U>; N]);
impl<const N: usize, T, L, U> std::ops::Deref for IntervalPow<N, T, L, U> {
    type Target = [Interval<T, L, U>; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize, T, L, U> std::ops::DerefMut for IntervalPow<N, T, L, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<const N: usize, T, L, U> From<[Interval<T, L, U>; N]> for IntervalPow<N, T, L, U> {
    fn from(intervals: [Interval<T, L, U>; N]) -> Self {
        Self(intervals)
    }
}
impl<const N: usize, T: Ord + Clone, L: Boundary, U: Boundary> IntervalPow<N, T, L, U> {
    pub fn lower(&self) -> [&Lower<T, L>; N] {
        std::array::from_fn(|i| self[i].lower())
    }

    pub fn upper(&self) -> [&Upper<T, U>; N] {
        std::array::from_fn(|i| self[i].upper())
    }

    pub fn min_val(&self) -> [T; N]
    where
        Lower<T, L>: crate::core::MinVal<T>,
    {
        std::array::from_fn(|i| self[i].min_val())
    }

    pub fn max_val(&self) -> [T; N]
    where
        Upper<T, U>: crate::core::MaxVal<T>,
    {
        std::array::from_fn(|i| self[i].max_val())
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let mut dst = self.clone();
        for i in 0..N {
            dst[i] = dst[i].clone().intersection(other[i].clone())?;
        }
        Some(dst)
    }

    pub fn union(&self, other: &Self) -> Self {
        Self(std::array::from_fn(|i| {
            self[i].clone().union(other[i].clone())
        }))
    }

    pub fn bound<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.union(&item.into())))
    }
}

impl<const N: usize, T: FloatCore, L: Boundary, U: Boundary> IntervalPow<N, NotNan<T>, L, U> {
    pub fn inf(&self) -> [NotNan<T>; N] {
        std::array::from_fn(|i| self[i].inf())
    }
    pub fn sup(&self) -> [NotNan<T>; N] {
        std::array::from_fn(|i| self[i].sup())
    }
    pub fn center(&self) -> [NotNan<T>; N] {
        std::array::from_fn(|i| self[i].center())
    }
    pub fn measure(&self) -> NotNan<T> {
        self.iter()
            .map(|i| i.measure())
            .fold(NotNan::new(T::one()).unwrap(), |a, b| a * b)
    }
    pub fn closure(self) -> IntervalPow<N, NotNan<T>, Inclusive> {
        IntervalPow(std::array::from_fn(|i| self[i].closure()))
    }
    pub fn interior(self) -> IntervalPow<N, NotNan<T>, Exclusive> {
        IntervalPow(std::array::from_fn(|i| self[i].interior()))
    }
}