mod boundary;
mod converters;
pub mod core;
mod impl_range_bounds;
mod ndim;
mod pow;
mod tests;

use ordered_float::{FloatCore, FloatIsNan, NotNan};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Inclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Exclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bound {
    Inclusive,
    Exclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lower<T, B> {
    pub val: T,
    pub bound: B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Upper<T, B> {
    pub val: T,
    pub bound: B,
}

pub trait IntoNotNanBound<B: boundary::Boundary> {
    type Float: FloatCore;
    fn into_not_nan_boundary(self) -> Result<(NotNan<Self::Float>, B), FloatIsNan>;
}

#[derive(Debug, thiserror::Error)]
#[error("lower boundary must be less than or equal to upper boundary")]
pub struct IntervalIsEmpty;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("float is NaN")]
    FloatIsNan(#[from] ordered_float::FloatIsNan),
    #[error("lower boundary must be less than or equal to upper boundary")]
    IntervalIsEmpty(#[from] IntervalIsEmpty),
}

pub use core::Interval;
pub type ClosedInterval<T> = Interval<T, Inclusive>;
pub type OpenInterval<T> = Interval<T, Exclusive>;
pub type RightHalfOpenInterval<T> = Interval<T, Inclusive, Exclusive>;
pub type LeftHalfOpenInterval<T> = Interval<T, Exclusive, Inclusive>;

pub type IntervalF<T, L, U> = Interval<NotNan<T>, L, U>;
pub type ClosedIntervalF<T> = ClosedInterval<NotNan<T>>;
pub type OpenIntervalF<T> = OpenInterval<NotNan<T>>;
pub type RightHalfOpenIntervalF<T> = RightHalfOpenInterval<NotNan<T>>;
pub type LeftHalfOpenIntervalF<T> = LeftHalfOpenInterval<NotNan<T>>;

pub type ClosedIntervalF64 = ClosedIntervalF<f64>;
pub type OpenIntervalF64 = OpenIntervalF<f64>;
pub type RightHalfOpenIntervalF64 = RightHalfOpenIntervalF<f64>;
pub type LeftHalfOpenIntervalF64 = LeftHalfOpenIntervalF<f64>;

pub use ndim::NDim;
pub type IntervalN<const N: usize, T, L = Bound, U = L> = NDim<N, Interval<T, L, U>>;
pub type Interval2<T, L = Bound, U = L> = IntervalN<2, T, L, U>;
pub type Interval3<T, L = Bound, U = L> = IntervalN<3, T, L, U>;
pub type Interval4<T, L = Bound, U = L> = IntervalN<4, T, L, U>;
pub type BoxN<const N: usize, T> = IntervalN<N, NotNan<T>, Inclusive>;
