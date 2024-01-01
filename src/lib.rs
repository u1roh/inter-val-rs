mod boundary;
mod converters;
pub mod core;
mod impl_range_bounds;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lower<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Upper<T>(pub T);

#[derive(Debug, thiserror::Error)]
#[error("lower boundary must be less than or equal to upper boundary")]
pub struct IntervalIsEmpty;

pub use core::Interval;
pub type ClosedInterval<T> = Interval<Inclusive<T>>;
pub type OpenInterval<T> = Interval<Exclusive<T>>;
pub type RightHalfOpenInterval<T> = Interval<Inclusive<T>, Exclusive<T>>;
pub type LeftHalfOpenInterval<T> = Interval<Exclusive<T>, Inclusive<T>>;
pub type GeneralInterval<T> = Interval<Boundary<T>>;

pub type ClosedIntervalF<T> = ClosedInterval<NotNan<T>>;
pub type OpenIntervalF<T> = OpenInterval<NotNan<T>>;
pub type RightHalfOpenIntervalF<T> = RightHalfOpenInterval<NotNan<T>>;
pub type LeftHalfOpenIntervalF<T> = LeftHalfOpenInterval<NotNan<T>>;

pub type ClosedIntervalF64 = ClosedIntervalF<f64>;
pub type OpenIntervalF64 = OpenIntervalF<f64>;
pub type RightHalfOpenIntervalF64 = RightHalfOpenIntervalF<f64>;
pub type LeftHalfOpenIntervalF64 = LeftHalfOpenIntervalF<f64>;
