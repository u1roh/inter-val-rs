//! Mathematical intervals, i.g., [a, b], (a, b), [a, b), and (a, b].
//!
//! # Usage
//! ```
//! use kd_interval::{Inclusive, Exclusive, Interval};
//!
//! // Closed interval of i32
//! let a = Inclusive.at(0).to(Inclusive.at(10));  // [0, 10]
//! let b = Inclusive.at(5).to(Inclusive.at(15));  // [5, 15]
//! let c = a.intersection(b).unwrap(); // [0, 10] ∩ [5, 15] = [5, 10]
//! assert_eq!(c.min(), 5);
//! assert_eq!(c.max(), 10);
//!
//! // Half-open interval of f64
//! let a = Inclusive.at(1.23).to(Exclusive.at(4.56));   // [1.23, 4.56)
//! assert_eq!(a.inf(), 1.23);
//! assert_eq!(a.sup(), 4.56);
//! assert!(a.contains(&1.23));
//! assert!(!a.contains(&4.56));
//! assert!(a.contains(&(4.56 - 0.000000000000001)));
//!
//! // Enclosure
//! let a = Interval::enclosure(vec![3, 9, 2, 5]).unwrap(); // [2, 9]
//! assert_eq!(a.min(), 2);
//! assert_eq!(a.max(), 9);
//! ```
mod bound;
mod bound_type;
mod converters;
mod half;
mod interval;
mod interval_box;
mod kd;
mod std_range;
mod tests;
mod traits;

use bound_type::{Left, Right};
use traits::BoundaryOf;

pub use bound::Bound;
pub use bound_type::{BoundType, Exclusive, Inclusive};
pub use half::{HalfBounded, LeftBounded, RightBounded};
pub use interval::Interval;
pub use interval_box::Box;
pub use kd::Kd;

impl Inclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            limit: t,
            bound_type: self,
        }
    }
    pub fn between<T: PartialOrd>(a: T, b: T) -> Interval<T, Self, Self> {
        Interval::between(a, b)
    }
}
impl Exclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            limit: t,
            bound_type: self,
        }
    }
    pub fn try_between<T: PartialOrd>(a: T, b: T) -> Option<Interval<T, Self, Self>> {
        Interval::try_between(a, b)
    }
}
impl BoundType {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            limit: t,
            bound_type: self,
        }
    }
}

impl<T: PartialOrd, B: BoundaryOf<Left>> Bound<T, B> {
    pub fn try_to<R: BoundaryOf<Right>>(self, right: Bound<T, R>) -> Option<Interval<T, B, R>> {
        Interval::try_new(self, right)
    }
    pub fn to<R: BoundaryOf<Right>>(self, right: Bound<T, R>) -> Interval<T, B, R> {
        Interval::new(self, right)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("left boundary must be less than or equal to right boundary")]
pub struct IntervalIsEmpty;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("infallible")]
    Infallible(#[from] std::convert::Infallible),
    #[error("left boundary must be less than or equal to right boundary")]
    IntervalIsEmpty(#[from] IntervalIsEmpty),
}

pub type ClosedInterval<T> = Interval<T, Inclusive>;
pub type OpenInterval<T> = Interval<T, Exclusive>;
