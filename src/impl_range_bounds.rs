use crate::{Exclusive, Inclusive, Lower, Upper};
use std::ops::{Bound, RangeBounds};

impl<T: Ord> RangeBounds<T> for Lower<T, Inclusive> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.val)
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
}
impl<T: Ord> RangeBounds<T> for Lower<T, Exclusive> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Excluded(&self.val)
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
}
impl<T: Ord> RangeBounds<T> for Upper<T, Inclusive> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(&self.val)
    }
}
impl<T: Ord> RangeBounds<T> for Upper<T, Exclusive> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Excluded(&self.val)
    }
}
