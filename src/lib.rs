//! Mathematical intervals, i.g., [a, b], (a, b), [a, b), and (a, b].
//!
//! # Usage
//! ```
//! use kd_interval::{Inclusive, Exclusive, Interval};
//!
//! // Closed interval of i32
//! let a = Inclusive.at(0).to(Inclusive.at(10)).unwrap();  // [0, 10]
//! let b = Inclusive.at(5).to(Inclusive.at(15)).unwrap();  // [5, 15]
//! let c = a.intersection(b).unwrap(); // [0, 10] ∩ [5, 15] = [5, 10]
//! assert_eq!(c.min(), 5);
//! assert_eq!(c.max(), 10);
//!
//! // Half-open interval of f64
//! let a = Inclusive.at(1.23).float_to(Exclusive.at(4.56)).unwrap();   // [1.23, 4.56)
//! assert_eq!(a.inf(), 1.23);
//! assert_eq!(a.sup(), 4.56);
//! assert!(a.contains(&1.23));
//! assert!(!a.contains(&4.56));
//! assert!(a.contains(&(4.56 - 0.000000000000001)));
//!
//! // Enclosure
//! let a = Interval::enclosure_of_items(vec![3, 9, 2, 5]).unwrap(); // [2, 9]
//! assert_eq!(a.min(), 2);
//! assert_eq!(a.max(), 9);
//! ```
mod bound;
mod bounding;
mod converters;
mod half;
mod interval;
mod ndim;
mod pow;
mod std_range;
mod tests;
mod traits;

use bounding::{Left, Right};
use ordered_float::{FloatCore, NotNan};
use traits::BoundaryOf;

// re-export ordered_float
pub use ordered_float;

pub use bound::Bound;
pub use bounding::{Bounding, Exclusive, Inclusive};
pub use half::{HalfBounded, LeftBounded, RightBounded};
pub use interval::Interval;
pub use traits::Scalar;

// Hidden for now
// pub use ndim::NDim;

impl Inclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            limit: t,
            bounding: self,
        }
    }
}
impl Exclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            limit: t,
            bounding: self,
        }
    }
}
impl Bounding {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            limit: t,
            bounding: self,
        }
    }
}

impl<T: Ord, B: BoundaryOf<Left>> Bound<T, B> {
    pub fn to<R: BoundaryOf<Right>>(self, right: Bound<T, R>) -> Option<Interval<T, B, R>> {
        Interval::new(self, right)
    }
}

impl<T: FloatCore, B: BoundaryOf<Left>> Bound<T, B> {
    pub fn float_to<R: BoundaryOf<Right>>(
        self,
        right: Bound<T, R>,
    ) -> Result<Interval<NotNan<T>, B, R>, Error> {
        Interval::try_new(self, right)?.ok_or(IntervalIsEmpty.into())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("left boundary must be less than or equal to right boundary")]
pub struct IntervalIsEmpty;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("infallible")]
    Infallible(#[from] std::convert::Infallible),
    #[error("float is NaN")]
    FloatIsNan(#[from] ordered_float::FloatIsNan),
    #[error("left boundary must be less than or equal to right boundary")]
    IntervalIsEmpty(#[from] IntervalIsEmpty),
}

pub type ClosedInterval<T> = Interval<T, Inclusive>;
pub type OpenInterval<T> = Interval<T, Exclusive>;

pub type IntervalF<T, L = Bounding, R = L> = Interval<NotNan<T>, L, R>;
pub type ClosedIntervalF<T> = ClosedInterval<NotNan<T>>;
pub type OpenIntervalF<T> = OpenInterval<NotNan<T>>;

// pub type IntervalN<const N: usize, T, L = Bounding, R = L> = NDim<N, Interval<T, L, R>>;
// pub type Interval2<T, L = Bounding, R = L> = IntervalN<2, T, L, R>;
// pub type Interval3<T, L = Bounding, R = L> = IntervalN<3, T, L, R>;
// pub type Interval4<T, L = Bounding, R = L> = IntervalN<4, T, L, R>;
// pub type BoxN<const N: usize, T> = IntervalN<N, NotNan<T>, Inclusive>;
