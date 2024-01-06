use crate::bound_type::{Left, Right};
use crate::kd::Kd;
use crate::traits::{BoundaryOf, Maximum, Minimum};
use crate::{Exclusive, Inclusive, Interval, LeftBounded, RightBounded};

#[derive(Debug, Clone, Copy, Eq)]
pub struct Box<const N: usize, T, L = Inclusive, R = L>(Kd<N, Interval<T, L, R>>);

impl<const N: usize, T: Eq, L: Eq, R: Eq> PartialEq for Box<N, T, L, R> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const N: usize, T, L, R> std::ops::Deref for Box<N, T, L, R> {
    type Target = Kd<N, Interval<T, L, R>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize, T, L, R> std::ops::DerefMut for Box<N, T, L, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize, T, L, R> From<[Interval<T, L, R>; N]> for Box<N, T, L, R> {
    fn from(items: [Interval<T, L, R>; N]) -> Self {
        Self(items.into())
    }
}

impl<const N: usize, T: PartialOrd + Clone, L: BoundaryOf<Left>, R: BoundaryOf<Right>>
    Box<N, T, L, R>
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

    pub fn min(&self) -> Kd<N, T>
    where
        LeftBounded<T, L>: Minimum<T>,
    {
        std::array::from_fn(|i| self[i].min()).into()
    }

    pub fn max(&self) -> Kd<N, T>
    where
        RightBounded<T, R>: Maximum<T>,
    {
        std::array::from_fn(|i| self[i].max()).into()
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let mut dst = self.clone();
        for i in 0..N {
            dst[i] = dst[i].clone().intersection(&other[i])?;
        }
        Some(dst)
    }

    pub fn span(&self, other: &Self) -> Self {
        std::array::from_fn(|i| self[i].clone().span(&other[i])).into()
    }

    pub fn span_many<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.span(&item.into())))
    }
}

impl<const N: usize, T: num::Float, L: BoundaryOf<Left>, R: BoundaryOf<Right>> Box<N, T, L, R> {
    pub fn inf(&self) -> Kd<N, &T> {
        std::array::from_fn(|i| self[i].inf()).into()
    }
    pub fn sup(&self) -> Kd<N, &T> {
        std::array::from_fn(|i| self[i].sup()).into()
    }
    pub fn center(&self) -> Kd<N, T> {
        std::array::from_fn(|i| self[i].center()).into()
    }
    pub fn size(&self) -> Kd<N, T> {
        std::array::from_fn(|i| self[i].measure()).into()
    }
    pub fn measure(&self) -> T {
        self.iter()
            .map(|item| item.measure())
            .fold(T::one(), |a, b| a * b)
    }
    pub fn closure(self) -> Kd<N, Interval<T, Inclusive>> {
        std::array::from_fn(|i| self[i].closure()).into()
    }
    pub fn interior(self) -> Option<Kd<N, Interval<T, Exclusive>>> {
        let interiors: [_; N] = std::array::from_fn(|i| self[i].interior());
        interiors
            .iter()
            .all(|i| i.is_some())
            .then(|| std::array::from_fn(|i| interiors[i].unwrap()).into())
    }
}
