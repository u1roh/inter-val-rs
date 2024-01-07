use crate::{
    bound_type::{Left, Right},
    traits::BoundaryOf,
    Bound, Interval,
};

/// Wrapper of `Option<T>` to implement `Sum` trait.
/// ```
/// use inter_val::{Nullable, Interval, Inclusive, Exclusive};
/// let a = Inclusive.at(0).to(Exclusive.at(3));  // [0, 3)
/// let b = Inclusive.at(1).to(Exclusive.at(5));  // [1, 5)
/// let c = Inclusive.at(8).to(Exclusive.at(10)); // [8, 10)
/// let span: Nullable<Interval<_, _, _>> = vec![a, b, c].into_iter().sum(); // [0, 10)
/// assert_eq!(span.as_ref().unwrap().left().limit, 0);
/// assert_eq!(span.as_ref().unwrap().right().limit, 10);
///
/// let hull: Nullable<Interval<i32>> = vec![1, 6, 2, 8, 3].into_iter().sum();
/// assert_eq!(hull.unwrap(), Interval::between(1, 8));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Nullable<T>(Option<T>);

impl<T> std::ops::Deref for Nullable<T> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> std::ops::DerefMut for Nullable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(o: Option<T>) -> Self {
        Self(o)
    }
}
impl<T> From<T> for Nullable<T> {
    fn from(t: T) -> Self {
        Self(Some(t))
    }
}
impl<T> From<Nullable<T>> for Option<T> {
    fn from(n: Nullable<T>) -> Self {
        n.0
    }
}

impl<T> Nullable<T> {
    pub const NULL: Self = Self(None);

    pub fn into_option(self) -> Option<T> {
        self.0
    }
    pub fn from_option(o: Option<T>) -> Self {
        Self(o)
    }
    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }
    pub fn unwrap(self) -> T {
        self.0.unwrap()
    }
}

/// ```
/// use inter_val::{Nullable, Interval, Inclusive, Exclusive};
/// let a = Inclusive.at(0).to(Exclusive.at(3));  // [0, 3)
/// let b = Inclusive.at(1).to(Exclusive.at(5));  // [1, 5)
/// let c = Inclusive.at(8).to(Exclusive.at(10)); // [8, 10)
/// let span: Nullable<Interval<_, _, _>> = vec![a, b, c].into_iter().sum(); // [0, 10)
/// assert_eq!(span.as_ref().unwrap().left().limit, 0);
/// assert_eq!(span.as_ref().unwrap().right().limit, 10);
/// ```
impl<T, L, R> std::iter::Sum<Interval<T, L, R>> for Nullable<Interval<T, L, R>>
where
    T: PartialOrd + Clone,
    L: BoundaryOf<Left>,
    R: BoundaryOf<Right>,
{
    fn sum<I: Iterator<Item = Interval<T, L, R>>>(iter: I) -> Self {
        Interval::span_many(iter).into()
    }
}

/// ```
/// use inter_val::{Interval, Nullable};
/// let a: Nullable<Interval<i32>> = vec![1, 6, 2, 8, 3].into_iter().sum();
/// assert_eq!(a.unwrap(), Interval::between(1, 8));
/// ```
impl<T, L, R> std::iter::Sum<T> for Nullable<Interval<T, L, R>>
where
    T: PartialOrd + Clone + Into<Bound<T, L>> + Into<Bound<T, R>>,
    L: BoundaryOf<Left>,
    R: BoundaryOf<Right>,
{
    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
        Interval::hull_many(iter).into()
    }
}
