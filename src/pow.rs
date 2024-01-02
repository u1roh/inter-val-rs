use ordered_float::{FloatCore, NotNan};

use crate::{boundary::Boundary, Interval};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalPow<const N: usize, T, L, U>([Interval<T, L, U>; N]);
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
    pub fn lowers(&self) -> [T; N] {
        std::array::from_fn(|i| self[i].lower().val.clone())
    }

    pub fn uppers(&self) -> [T; N] {
        std::array::from_fn(|i| self[i].upper().val.clone())
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
    pub fn measure(&self) -> NotNan<T> {
        self.iter()
            .map(|i| i.measure())
            .fold(NotNan::new(T::one()).unwrap(), |a, b| a * b)
    }
}
