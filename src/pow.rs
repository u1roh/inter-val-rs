use ordered_float::{FloatCore, NotNan};

use crate::bound_type::{Left, Right};
use crate::kd::Kd;
use crate::traits::{BoundaryOf, Maximum, Minimum};
use crate::{Exclusive, Inclusive, Interval, LeftBounded, RightBounded};

impl<const N: usize, T: Ord + Clone, L: BoundaryOf<Left>, R: BoundaryOf<Right>>
    Kd<N, Interval<T, L, R>>
{
    pub fn left(&self) -> Kd<N, &LeftBounded<T, L>> {
        std::array::from_fn(|i| self[i].left()).into()
    }

    pub fn right(&self) -> Kd<N, &RightBounded<T, R>> {
        std::array::from_fn(|i| self[i].right()).into()
    }

    pub fn contains(&self, t: &Kd<N, T>) -> bool {
        self.iter().zip(t.iter()).all(|(i, t)| i.contains(t))
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(i, o)| i.includes(o))
    }

    pub fn min_val(&self) -> Kd<N, T>
    where
        LeftBounded<T, L>: Minimum<T>,
    {
        std::array::from_fn(|i| self[i].min()).into()
    }

    pub fn max_val(&self) -> Kd<N, T>
    where
        RightBounded<T, R>: Maximum<T>,
    {
        std::array::from_fn(|i| self[i].max()).into()
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
            self[i].clone().hull(other[i].clone())
        }))
    }

    pub fn bound<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.union(&item.into())))
    }
}

impl<const N: usize, T: FloatCore, L: BoundaryOf<Left>, R: BoundaryOf<Right>>
    Kd<N, Interval<NotNan<T>, L, R>>
{
    pub fn inf(&self) -> Kd<N, NotNan<T>> {
        std::array::from_fn(|i| self[i].inf()).into()
    }
    pub fn sup(&self) -> Kd<N, NotNan<T>> {
        std::array::from_fn(|i| self[i].sup()).into()
    }
    pub fn center(&self) -> Kd<N, T> {
        std::array::from_fn(|i| self[i].center()).into()
    }
    pub fn size(&self) -> Kd<N, T> {
        std::array::from_fn(|i| self[i].measure()).into()
    }
    pub fn measure(&self) -> NotNan<T> {
        self.iter()
            .map(|item| item.measure())
            .fold(NotNan::new(T::one()).unwrap(), |a, b| a * b)
    }
    pub fn closure(self) -> Kd<N, Interval<NotNan<T>, Inclusive>> {
        std::array::from_fn(|i| self[i].closure()).into()
    }
    pub fn interior(self) -> Option<Kd<N, Interval<NotNan<T>, Exclusive>>> {
        let interiors: [_; N] = std::array::from_fn(|i| self[i].interior());
        interiors
            .iter()
            .all(|i| i.is_some())
            .then(|| std::array::from_fn(|i| interiors[i].unwrap()).into())
    }
}
