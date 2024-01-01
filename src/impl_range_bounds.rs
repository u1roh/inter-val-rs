use crate::{Exclusive, Inclusive, Lower, Upper};

impl<T: Ord> std::ops::RangeBounds<T> for Lower<Inclusive<T>> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Included(self.inf())
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Unbounded
    }
}
impl<T: Ord> std::ops::RangeBounds<T> for Lower<Exclusive<T>> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Excluded(self.inf())
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Unbounded
    }
}
impl<T: Ord> std::ops::RangeBounds<T> for Upper<Inclusive<T>> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Unbounded
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Included(self.sup())
    }
}
impl<T: Ord> std::ops::RangeBounds<T> for Upper<Exclusive<T>> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Unbounded
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Excluded(self.sup())
    }
}
