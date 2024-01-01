mod boundary;
mod converters;
pub mod core;
mod impl_range_bounds;

use ordered_float::NotNan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Exclusive;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub enum (T, Bound) {
//     Inclusive(T),
//     Exclusive(T),
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bound {
    Inclusive,
    Exclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lower<T, B> {
    val: T,
    bound: B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Upper<T, B> {
    val: T,
    bound: B,
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

// impl<T: ordered_float::FloatCore> Inclusive<NotNan<T>> {
//     pub fn not_nan(t: T) -> Result<Self, ordered_float::FloatIsNan> {
//         NotNan::new(t).map(Self)
//     }
// }
// impl<T: ordered_float::FloatCore> Exclusive<NotNan<T>> {
//     pub fn not_nan(t: T) -> Result<Self, ordered_float::FloatIsNan> {
//         NotNan::new(t).map(Self)
//     }
// }

// impl<T: Ord> (T, Inclusive) {
//     pub fn to<B: Boundary<Val = T>>(self, end: B) -> Result<Interval<Self, B>, IntervalIsEmpty> {
//         Interval::new(self, end)
//     }
// }
// impl<T: Ord> (T, Exclusive) {
//     pub fn to<B: Boundary<Val = T>>(self, end: B) -> Result<Interval<Self, B>, IntervalIsEmpty> {
//         Interval::new(self, end)
//     }
// }
// impl<T: Ord> (T, Bound) {
//     pub fn to(self, end: (T, Bound)) -> Result<Interval<Self>, IntervalIsEmpty> {
//         Interval::new(self, end)
//     }
// }

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

// TODO
pub struct Box<const N: usize, L, U>([Interval<L, U>; N]);
