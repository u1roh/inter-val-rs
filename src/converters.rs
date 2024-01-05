use crate::{traits::IntoGeneral, Bound, Exclusive, Inclusive, Interval};

impl<T> From<T> for Bound<T, Inclusive> {
    fn from(t: T) -> Self {
        Self {
            val: t,
            bounding: Inclusive,
        }
    }
}
impl<T> From<T> for Bound<T, Exclusive> {
    fn from(t: T) -> Self {
        Self {
            val: t,
            bounding: Exclusive,
        }
    }
}

/// ```
/// use intervals::{Bounding, Inclusive, Interval};
/// let src: Interval<i32, Inclusive> = Inclusive.at(0).to(Inclusive.at(10)).unwrap();
/// let dst: Interval<i32> = src.into();
/// assert_eq!(dst.left().bounding, Bounding::Inclusive);
/// assert_eq!(dst.right().bounding, Bounding::Inclusive);
/// ```
impl<T> From<Interval<T, Inclusive>> for Interval<T> {
    fn from(i: Interval<T, Inclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use intervals::{Bounding, Exclusive, Interval};
/// let src: Interval<i32, Exclusive> = Exclusive.at(0).to(Exclusive.at(10)).unwrap();
/// let dst: Interval<i32> = src.into();
/// assert_eq!(dst.left().bounding, Bounding::Exclusive);
/// assert_eq!(dst.right().bounding, Bounding::Exclusive);
/// ```
impl<T> From<Interval<T, Exclusive>> for Interval<T> {
    fn from(i: Interval<T, Exclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use intervals::{Bounding, Inclusive, Exclusive, Interval};
/// let src: Interval<i32, Inclusive, Exclusive> = Inclusive.at(0).to(Exclusive.at(10)).unwrap();
/// let dst: Interval<i32> = src.into();
/// assert_eq!(dst.left().bounding, Bounding::Inclusive);
/// assert_eq!(dst.right().bounding, Bounding::Exclusive);
/// ```
impl<T> From<Interval<T, Inclusive, Exclusive>> for Interval<T> {
    fn from(i: Interval<T, Inclusive, Exclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use intervals::{Bounding, Inclusive, Exclusive, Interval};
/// let src: Interval<i32, Exclusive, Inclusive> = Exclusive.at(0).to(Inclusive.at(10)).unwrap();
/// let dst: Interval<i32> = src.into();
/// assert_eq!(dst.left().bounding, Bounding::Exclusive);
/// assert_eq!(dst.right().bounding, Bounding::Inclusive);
/// ```
impl<T> From<Interval<T, Exclusive, Inclusive>> for Interval<T> {
    fn from(i: Interval<T, Exclusive, Inclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use std::any::{Any, TypeId};
/// use intervals::{Inclusive, Interval};
/// let a: Interval<_, _, _> = 3.into();
/// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Inclusive, Inclusive>>());
/// assert_eq!(a.left().val, 3);
/// assert_eq!(a.right().val, 3);
impl<T: Ord + Clone> From<T> for Interval<T, Inclusive> {
    fn from(t: T) -> Self {
        Self::new(t.clone().into(), t.into()).unwrap()
    }
}
