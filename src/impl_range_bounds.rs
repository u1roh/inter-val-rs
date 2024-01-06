use crate::{Exclusive, Inclusive, LeftBounded, RightBounded};
use std::ops::{Bound, RangeBounds};

impl<T: Ord> RangeBounds<T> for LeftBounded<T, Inclusive> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.limit)
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
}
impl<T: Ord> RangeBounds<T> for LeftBounded<T, Exclusive> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Excluded(&self.limit)
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
}
impl<T: Ord> RangeBounds<T> for RightBounded<T, Inclusive> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(&self.limit)
    }
}
impl<T: Ord> RangeBounds<T> for RightBounded<T, Exclusive> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Excluded(&self.limit)
    }
}
