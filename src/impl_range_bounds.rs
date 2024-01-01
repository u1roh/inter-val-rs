use crate::{boundary::Boundary, Exclusive, Inclusive, Interval, Lower, Upper};
use std::ops::{Bound, RangeBounds};

impl<T: Ord> RangeBounds<T> for Lower<Inclusive<T>> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(self.inf())
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
}
impl<T: Ord> RangeBounds<T> for Lower<Exclusive<T>> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Excluded(self.inf())
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
}
impl<T: Ord> RangeBounds<T> for Upper<Inclusive<T>> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(self.sup())
    }
}
impl<T: Ord> RangeBounds<T> for Upper<Exclusive<T>> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Excluded(self.sup())
    }
}
impl<L: Boundary, U: Boundary<Val = L::Val>> RangeBounds<L::Val> for Interval<L, U> {
    fn start_bound(&self) -> Bound<&L::Val> {
        Bound::Included(self.inf())
    }
    fn end_bound(&self) -> Bound<&L::Val> {
        Bound::Excluded(self.sup())
    }
}
