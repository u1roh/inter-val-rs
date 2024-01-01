mod converters;
pub mod core;

use ordered_float::NotNan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inclusive<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Exclusive<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Boundary<T> {
    Inclusive(T),
    Exclusive(T),
}

#[derive(Debug, thiserror::Error)]
#[error("lower boundary must be less than or equal to upper boundary")]
pub struct IntervalIsEmpty;

pub type ClosedInterval<T> = core::Interval<Inclusive<T>>;
pub type OpenInterval<T> = core::Interval<Exclusive<T>>;
pub type RightHalfOpenInterval<T> = core::Interval<Inclusive<T>, Exclusive<T>>;
pub type LeftHalfOpenInterval<T> = core::Interval<Exclusive<T>, Inclusive<T>>;
pub type Interval<T> = core::Interval<Boundary<T>>;

pub type ClosedIntervalF<T> = ClosedInterval<NotNan<T>>;
pub type OpenIntervalF<T> = OpenInterval<NotNan<T>>;
pub type RightHalfOpenIntervalF<T> = RightHalfOpenInterval<NotNan<T>>;
pub type LeftHalfOpenIntervalF<T> = LeftHalfOpenInterval<NotNan<T>>;

pub type ClosedIntervalF64 = ClosedIntervalF<f64>;
pub type OpenIntervalF64 = OpenIntervalF<f64>;
pub type RightHalfOpenIntervalF64 = RightHalfOpenIntervalF<f64>;
pub type LeftHalfOpenIntervalF64 = LeftHalfOpenIntervalF<f64>;
