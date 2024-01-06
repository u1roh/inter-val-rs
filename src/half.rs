use ordered_float::{FloatCore, NotNan};

use crate::{
    bound_type::{Left, Right},
    traits::{Boundary, BoundaryOf, Flip, IntoGeneral, Maximum, Minimum, Scalar},
    Bound, BoundType, Exclusive, Inclusive,
};

#[derive(Debug, Clone, Copy)]
pub struct HalfBounded<T, B, LR>(pub(crate) Bound<T, B>, std::marker::PhantomData<LR>);

pub type LeftBounded<T, B> = HalfBounded<T, B, Left>;
pub type RightBounded<T, B> = HalfBounded<T, B, Right>;

impl<T, B, LR> std::ops::Deref for HalfBounded<T, B, LR> {
    type Target = Bound<T, B>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, B, LR> std::ops::DerefMut for HalfBounded<T, B, LR> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

mod ordering {
    use super::HalfBounded;
    use crate::traits::BoundaryOf;

    impl<T: Eq, B: Eq, LR> PartialEq for HalfBounded<T, B, LR> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<T: Eq, B: Eq, LR> Eq for HalfBounded<T, B, LR> {}

    impl<T: Ord, B: BoundaryOf<LR>, LR> HalfBounded<T, B, LR> {
        fn ordering_key(&self) -> (&T, B::Ordered) {
            (&self.limit, self.bound_type.into_ordered())
        }
    }
    impl<T: Ord, B: BoundaryOf<LR>, LR> PartialOrd for HalfBounded<T, B, LR> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<T: Ord, B: BoundaryOf<LR>, LR> Ord for HalfBounded<T, B, LR> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.ordering_key().cmp(&other.ordering_key())
        }
    }
}

impl<T, B, LR> From<Bound<T, B>> for HalfBounded<T, B, LR> {
    fn from(b: Bound<T, B>) -> Self {
        HalfBounded(b, std::marker::PhantomData)
    }
}

impl<T, B: IntoGeneral, LR> IntoGeneral for HalfBounded<T, B, LR> {
    type General = HalfBounded<T, B::General, LR>;
    fn into_general(self) -> Self::General {
        HalfBounded(self.0.into_general(), std::marker::PhantomData)
    }
}

impl<T, B: Flip, LR: Flip> Flip for HalfBounded<T, B, LR> {
    type Flip = HalfBounded<T, B::Flip, LR::Flip>;
    fn flip(self) -> Self::Flip {
        HalfBounded(self.0.flip(), std::marker::PhantomData)
    }
}

impl<T: Ord, B: BoundaryOf<Left>> LeftBounded<T, B> {
    pub fn includes(&self, other: &Self) -> bool {
        self.limit <= other.limit
    }
    pub fn contains<T2>(&self, t: &T2) -> bool
    where
        T: Scalar<T2>,
    {
        self.bound_type.less(&self.limit, t)
    }
    pub fn intersection(self, other: Self) -> Self {
        self.max(other)
    }
    pub fn union(self, other: Self) -> Self {
        self.min(other)
    }

    pub fn dilate(self, delta: T) -> Self
    where
        T: std::ops::Sub<Output = T>,
    {
        Bound {
            limit: self.0.limit - delta,
            bound_type: self.0.bound_type,
        }
        .into()
    }

    pub fn try_dilate<X>(self, delta: X) -> Result<Self, T::Error>
    where
        T: Scalar<X>,
        X: std::ops::Sub<Output = X>,
    {
        Ok(Bound {
            limit: T::scalar_try_from(self.0.limit.scalar_into() - delta)?,
            bound_type: self.0.bound_type,
        }
        .into())
    }
}

impl<T: Ord, B: BoundaryOf<Right>> RightBounded<T, B> {
    pub fn includes(&self, other: &Self) -> bool {
        other.limit <= self.limit
    }
    pub fn contains<T2>(&self, t: &T2) -> bool
    where
        T: Scalar<T2>,
    {
        self.bound_type.greater(&self.limit, t)
    }
    pub fn intersection(self, other: Self) -> Self {
        self.min(other)
    }
    pub fn union(self, other: Self) -> Self {
        self.max(other)
    }

    pub fn dilate(self, delta: T) -> Self
    where
        T: std::ops::Add<Output = T>,
    {
        Bound {
            limit: self.0.limit + delta,
            bound_type: self.0.bound_type,
        }
        .into()
    }

    pub fn try_dilate<X>(self, delta: X) -> Result<Self, T::Error>
    where
        T: Scalar<X>,
        X: std::ops::Add<Output = X>,
    {
        Ok(Bound {
            limit: T::scalar_try_from(self.0.limit.scalar_into() + delta)?,
            bound_type: self.0.bound_type,
        }
        .into())
    }
}

impl<T: Clone> Minimum<T> for LeftBounded<T, Inclusive> {
    fn minimum(&self) -> T {
        self.limit.clone()
    }
}
impl<T: Clone> Maximum<T> for RightBounded<T, Inclusive> {
    fn maximum(&self) -> T {
        self.limit.clone()
    }
}

impl<T: num::Integer + Clone> Minimum<T> for LeftBounded<T, Exclusive> {
    fn minimum(&self) -> T {
        self.limit.clone() + T::one()
    }
}
impl<T: num::Integer + Clone> Maximum<T> for RightBounded<T, Exclusive> {
    fn maximum(&self) -> T {
        self.limit.clone() - T::one()
    }
}

impl<T: num::Integer + Clone> Minimum<T> for LeftBounded<T, BoundType> {
    fn minimum(&self) -> T {
        match self.bound_type {
            BoundType::Inclusive => self.limit.clone(),
            BoundType::Exclusive => self.limit.clone() + T::one(),
        }
    }
}
impl<T: num::Integer + Clone> Maximum<T> for RightBounded<T, BoundType> {
    fn maximum(&self) -> T {
        match self.bound_type {
            BoundType::Inclusive => self.limit.clone(),
            BoundType::Exclusive => self.limit.clone() - T::one(),
        }
    }
}

impl<T: FloatCore, B: Boundary> LeftBounded<NotNan<T>, B> {
    pub fn inf(&self) -> NotNan<T> {
        self.limit
    }
    pub fn closure(self) -> LeftBounded<NotNan<T>, Inclusive> {
        Bound {
            limit: self.limit,
            bound_type: Inclusive,
        }
        .into()
    }
    pub fn interior(self) -> LeftBounded<NotNan<T>, Exclusive> {
        Bound {
            limit: self.limit,
            bound_type: Exclusive,
        }
        .into()
    }
}
impl<T: FloatCore, B: Boundary> RightBounded<NotNan<T>, B> {
    pub fn sup(&self) -> NotNan<T> {
        self.limit
    }
    pub fn closure(self) -> RightBounded<NotNan<T>, Inclusive> {
        Bound {
            limit: self.limit,
            bound_type: Inclusive,
        }
        .into()
    }
    pub fn interior(self) -> RightBounded<NotNan<T>, Exclusive> {
        Bound {
            limit: self.limit,
            bound_type: Exclusive,
        }
        .into()
    }
}
