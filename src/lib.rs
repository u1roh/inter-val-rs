//! Mathematical intervals, i.g., [a, b], (a, b), [a, b), and (a, b].
//! Also supports multi-dimensional axis-aligned boxes.
//!
//! # Interval
//! Intervals like *[a, b]*, *(a, b)*, *[a, b)*, and *(a, b]* for any `PartialOrd` type.
//!
//! ```
//! use inter_val::{Inclusive, Exclusive, Interval};
//!
//! // Closed interval of i32
//! let a = Inclusive.at(0).to(Inclusive.at(10));  // [0, 10]
//! assert!(a.contains(&3));
//!
//! // Half-open interval of f64
//! let b = Inclusive.at(1.23).to(Exclusive.at(4.56));   // [1.23, 4.56)
//! assert!(!b.contains(&4.56));
//! assert!(b.contains(&(4.56 - 0.000000000000001)));
//!
//! // Intersection
//! let c = Inclusive.between(5, 15);  // [5, 15]
//! let isect = a.intersection(&c).unwrap(); // [0, 10] ∩ [5, 15] = [5, 10]
//! assert_eq!(isect.inf(), &5);
//! assert_eq!(isect.sup(), &10);
//!
//! // Span & Gap
//! let d = Inclusive.between(12, 15);  // [12, 15]
//! let span = a.span(&d);  // [0, 15]
//! let gap = a.gap(&d);    // (10, 12)
//! assert_eq!(span, Inclusive.between(0, 15));
//! assert_eq!(gap.unwrap(), Exclusive.between(10, 12));
//!
//! // Union
//! let union = a.union(&d);
//! assert_eq!(union.span, span);
//! assert_eq!(union.gap, gap);
//! assert_eq!(union.into_vec(), vec![a, d]);
//!
//! // Hull
//! let a = Interval::<_>::hull_many(vec![3, 9, 2, 5]).unwrap(); // [2, 9]
//! assert_eq!(a, Inclusive.between(2, 9));
//! ```
//!
//! # Multi-dimensional axis-aligned box
//! Boxes represented by Cartesian product of intervals.
//! ```
//! use inter_val::{Box2, Inclusive};
//!
//! // [0.0, 10.0] × [5.0, 20.0]
//! let a: Box2<f64> = Box2::new(Inclusive.between(0.0, 10.0), Inclusive.between(5.0, 20.0));
//!
//! // Another way to construct [0.0, 10.0] × [5.0, 20.0]
//! let b: Box2<f64> = Box2::between(&[0.0, 5.0], &[10.0, 20.0]);
//! assert_eq!(a, b);
//!
//! // Hull
//! let b = a.hull(&[12.3, 7.5]);
//! assert_eq!(b, Box2::between(&[0.0, 5.0], &[12.3, 20.0]));
//! ```
mod bound;
mod bound_type;
mod converters;
mod half;
mod interval;
mod interval_box;
mod ndim;
mod nullable;
mod std_range;
mod tests;
mod traits;

use bound_type::{Left, Right};
use traits::BoundaryOf;

pub use bound::Bound;
pub use bound_type::{BoundType, Exclusive, Inclusive};
pub use half::{HalfBounded, LeftBounded, RightBounded};
pub use interval::Interval;
pub use interval_box::BoxN;
pub use ndim::NDim;
pub use nullable::Nullable;

impl Inclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            limit: t,
            bound_type: self,
        }
    }
    pub fn between<T: PartialOrd>(self, a: T, b: T) -> Interval<T, Self, Self> {
        Interval::between(a, b)
    }
    pub fn hull<T: PartialOrd + Clone>(
        items: impl IntoIterator<Item = T>,
    ) -> Option<Interval<T, Self, Self>> {
        Interval::hull_many(items)
    }
}
impl Exclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            limit: t,
            bound_type: self,
        }
    }
    pub fn try_between<T: PartialOrd>(self, a: T, b: T) -> Option<Interval<T, Self, Self>> {
        Interval::try_between(a, b)
    }
    pub fn between<T: PartialOrd>(self, a: T, b: T) -> Interval<T, Self, Self> {
        Interval::between(a, b)
    }
    pub fn hull<T: PartialOrd + Clone>(
        items: impl IntoIterator<Item = T>,
    ) -> Option<Interval<T, Self, Self>> {
        Interval::hull_many(items)
    }
}
impl BoundType {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            limit: t,
            bound_type: self,
        }
    }
    pub fn try_between<T: PartialOrd>(self, a: T, b: T) -> Option<Interval<T, Self, Self>> {
        if a <= b {
            Interval::try_new(self.at(a), self.at(b))
        } else {
            Interval::try_new(self.at(b), self.at(a))
        }
    }
}

impl<T: PartialOrd, B: BoundaryOf<Left>> Bound<T, B> {
    pub fn to<R: BoundaryOf<Right>>(self, r: Bound<T, R>) -> Interval<T, B, R> {
        Interval::new(self, r)
    }
    pub fn try_to<R: BoundaryOf<Right>>(self, r: Bound<T, R>) -> Option<Interval<T, B, R>> {
        Interval::try_new(self, r)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("left boundary must be less than or equal to right boundary")]
pub struct IntervalIsEmpty;

pub type OpenInterval<T> = Interval<T, Exclusive>;
pub type GeneralInterval<T> = Interval<T, BoundType>;
pub type Box2<T, L = Inclusive, R = L> = BoxN<2, T, L, R>;
pub type Box3<T, L = Inclusive, R = L> = BoxN<3, T, L, R>;
pub type Box4<T, L = Inclusive, R = L> = BoxN<4, T, L, R>;
