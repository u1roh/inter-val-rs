mod boundary;
mod converters;
pub mod core;
mod half;
mod impl_range_bounds;
mod ndim;
mod pow;
mod tests;

use ordered_float::{FloatCore, FloatIsNan, NotNan};

pub use half::{HalfBounded, LeftBounded, RightBounded};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Inclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Exclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Inclusion {
    Inclusive,
    Exclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bound<T, B> {
    pub val: T,
    pub inclusion: B,
}
impl<T: Ord, B: boundary::Boundary> Bound<T, B> {
    pub fn to<R: boundary::Boundary>(
        self,
        right: Bound<T, R>,
    ) -> Result<Interval<T, B, R>, IntervalIsEmpty> {
        Interval::new(self, right)
    }
}
impl<T: FloatCore, B: boundary::Boundary> Bound<T, B> {
    pub fn not_nan_to<R: boundary::Boundary>(
        self,
        right: Bound<T, R>,
    ) -> Result<Interval<NotNan<T>, B, R>, Error> {
        Interval::not_nan(self, right)
    }
}
impl<T: FloatCore, B> Bound<T, B> {
    pub fn into_not_nan(self) -> Result<Bound<NotNan<T>, B>, FloatIsNan> {
        NotNan::new(self.val).map(|val| Bound {
            val,
            inclusion: self.inclusion,
        })
    }
}

impl Inclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            val: t,
            inclusion: self,
        }
    }
    pub fn not_nan<T: FloatCore>(self, t: T) -> Result<Bound<NotNan<T>, Self>, FloatIsNan> {
        self.at(t).into_not_nan()
    }
}
impl Exclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            val: t,
            inclusion: self,
        }
    }
    pub fn not_nan<T: FloatCore>(self, t: T) -> Result<Bound<NotNan<T>, Self>, FloatIsNan> {
        self.at(t).into_not_nan()
    }
}
impl Inclusion {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            val: t,
            inclusion: self,
        }
    }
    pub fn not_nan<T: FloatCore>(self, t: T) -> Result<Bound<NotNan<T>, Self>, FloatIsNan> {
        self.at(t).into_not_nan()
    }
}

pub trait Minimum<T> {
    fn minimum(&self) -> T;
}
pub trait Maximum<T> {
    fn maximum(&self) -> T;
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
pub type IntervalN<const N: usize, T, L = Inclusion, U = L> = NDim<N, Interval<T, L, U>>;
pub type Interval2<T, L = Inclusion, U = L> = IntervalN<2, T, L, U>;
pub type Interval3<T, L = Inclusion, U = L> = IntervalN<3, T, L, U>;
pub type Interval4<T, L = Inclusion, U = L> = IntervalN<4, T, L, U>;
pub type BoxN<const N: usize, T> = IntervalN<N, NotNan<T>, Inclusive>;
