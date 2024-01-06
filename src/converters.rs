use crate::{traits::IntoGeneral, Bound, Exclusive, Inclusive, Interval};

impl<T> From<T> for Bound<T, Inclusive> {
    fn from(t: T) -> Self {
        Self {
            limit: t,
            bound_type: Inclusive,
        }
    }
}
impl<T> From<T> for Bound<T, Exclusive> {
    fn from(t: T) -> Self {
        Self {
            limit: t,
            bound_type: Exclusive,
        }
    }
}

/// ```
/// use kd_interval::{BoundType, Inclusive, Interval};
/// let src: Interval<i32, Inclusive> = Inclusive.at(0).to(Inclusive.at(10)).unwrap();
/// let dst: Interval<i32> = src.into();
/// assert_eq!(dst.left().bound_type, BoundType::Inclusive);
/// assert_eq!(dst.right().bound_type, BoundType::Inclusive);
/// ```
impl<T> From<Interval<T, Inclusive>> for Interval<T> {
    fn from(i: Interval<T, Inclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use kd_interval::{BoundType, Exclusive, Interval};
/// let src: Interval<i32, Exclusive> = Exclusive.at(0).to(Exclusive.at(10)).unwrap();
/// let dst: Interval<i32> = src.into();
/// assert_eq!(dst.left().bound_type, BoundType::Exclusive);
/// assert_eq!(dst.right().bound_type, BoundType::Exclusive);
/// ```
impl<T> From<Interval<T, Exclusive>> for Interval<T> {
    fn from(i: Interval<T, Exclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use kd_interval::{BoundType, Inclusive, Exclusive, Interval};
/// let src: Interval<i32, Inclusive, Exclusive> = Inclusive.at(0).to(Exclusive.at(10)).unwrap();
/// let dst: Interval<i32> = src.into();
/// assert_eq!(dst.left().bound_type, BoundType::Inclusive);
/// assert_eq!(dst.right().bound_type, BoundType::Exclusive);
/// ```
impl<T> From<Interval<T, Inclusive, Exclusive>> for Interval<T> {
    fn from(i: Interval<T, Inclusive, Exclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use kd_interval::{BoundType, Inclusive, Exclusive, Interval};
/// let src: Interval<i32, Exclusive, Inclusive> = Exclusive.at(0).to(Inclusive.at(10)).unwrap();
/// let dst: Interval<i32> = src.into();
/// assert_eq!(dst.left().bound_type, BoundType::Exclusive);
/// assert_eq!(dst.right().bound_type, BoundType::Inclusive);
/// ```
impl<T> From<Interval<T, Exclusive, Inclusive>> for Interval<T> {
    fn from(i: Interval<T, Exclusive, Inclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use std::any::{Any, TypeId};
/// use kd_interval::{Inclusive, Interval};
/// let a: Interval<_, _, _> = 3.into();
/// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Inclusive, Inclusive>>());
/// assert_eq!(a.left().limit, 3);
/// assert_eq!(a.right().limit, 3);
impl<T: Ord + Clone> From<T> for Interval<T, Inclusive> {
    fn from(t: T) -> Self {
        Self::new(t.clone().into(), t.into()).unwrap()
    }
}
