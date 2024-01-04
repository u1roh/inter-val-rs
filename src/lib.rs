mod bound;
mod converters;
mod half;
mod impl_range_bounds;
mod inclusion;
mod interval;
mod ndim;
mod pow;
mod tests;

use half::{LeftInclusion, RightInclusion};
use ordered_float::{FloatCore, FloatIsNan, NotNan};

pub use bound::Bound;
pub use half::{LeftBounded, RightBounded};
pub use inclusion::{Exclusive, Inclusion, Inclusive};

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

impl<T: Ord, B: inclusion::Boundary> Bound<T, B>
where
    LeftInclusion<B>: Ord,
{
    pub fn to<R: inclusion::Boundary>(
        self,
        right: Bound<T, R>,
    ) -> Result<Interval<T, B, R>, IntervalIsEmpty>
    where
        RightInclusion<R>: Ord,
    {
        Interval::new(self, right)
    }
}

impl<T: FloatCore, B: inclusion::Boundary> Bound<T, B>
where
    LeftInclusion<B>: Ord,
{
    pub fn not_nan_to<R: inclusion::Boundary>(
        self,
        right: Bound<T, R>,
    ) -> Result<Interval<NotNan<T>, B, R>, Error>
    where
        RightInclusion<R>: Ord,
    {
        Interval::not_nan(self, right)
    }
}

pub trait Minimum<T> {
    fn minimum(&self) -> T;
}
pub trait Maximum<T> {
    fn maximum(&self) -> T;
}

#[derive(Debug, thiserror::Error)]
#[error("left boundary must be less than or equal to right boundary")]
pub struct IntervalIsEmpty;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("float is NaN")]
    FloatIsNan(#[from] ordered_float::FloatIsNan),
    #[error("left boundary must be less than or equal to right boundary")]
    IntervalIsEmpty(#[from] IntervalIsEmpty),
}

pub use interval::Interval;
pub type ClosedInterval<T> = Interval<T, Inclusive>;
pub type OpenInterval<T> = Interval<T, Exclusive>;
pub type RightHalfOpenInterval<T> = Interval<T, Inclusive, Exclusive>;
pub type LeftHalfOpenInterval<T> = Interval<T, Exclusive, Inclusive>;

pub type IntervalF<T, L, R> = Interval<NotNan<T>, L, R>;
pub type ClosedIntervalF<T> = ClosedInterval<NotNan<T>>;
pub type OpenIntervalF<T> = OpenInterval<NotNan<T>>;
pub type RightHalfOpenIntervalF<T> = RightHalfOpenInterval<NotNan<T>>;
pub type LeftHalfOpenIntervalF<T> = LeftHalfOpenInterval<NotNan<T>>;

pub type ClosedIntervalF64 = ClosedIntervalF<f64>;
pub type OpenIntervalF64 = OpenIntervalF<f64>;
pub type RightHalfOpenIntervalF64 = RightHalfOpenIntervalF<f64>;
pub type LeftHalfOpenIntervalF64 = LeftHalfOpenIntervalF<f64>;

pub use ndim::NDim;
pub type IntervalN<const N: usize, T, L = Inclusion, R = L> = NDim<N, Interval<T, L, R>>;
pub type Interval2<T, L = Inclusion, R = L> = IntervalN<2, T, L, R>;
pub type Interval3<T, L = Inclusion, R = L> = IntervalN<3, T, L, R>;
pub type Interval4<T, L = Inclusion, R = L> = IntervalN<4, T, L, R>;
pub type BoxN<const N: usize, T> = IntervalN<N, NotNan<T>, Inclusive>;
