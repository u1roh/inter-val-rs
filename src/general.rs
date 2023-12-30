#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound<T> {
    Inclusive(T),
    Exclusive(T),
}
pub use Bound::*;
impl<T> From<T> for Bound<T> {
    fn from(t: T) -> Self {
        Bound::Inclusive(t) // default to inclusive
    }
}
impl<T> std::ops::Deref for Bound<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Bound::Inclusive(t) => t,
            Bound::Exclusive(t) => t,
        }
    }
}

pub struct Interval<T> {
    left: Bound<T>,
    right: Bound<T>,
}
impl<T: PartialOrd> Interval<T> {
    pub fn new(left: Bound<T>, right: Bound<T>) -> Self {
        match (&left, &right) {
            (Inclusive(a), Inclusive(b)) => assert!(a <= b),
            _ => assert!(*left < *right),
        }
        Self { left, right }
    }
    pub fn left(&self) -> &Bound<T> {
        &self.left
    }
    pub fn right(&self) -> &Bound<T> {
        &self.right
    }
    pub fn contains(&self, t: &T) -> bool {
        match (&self.left, &self.right) {
            (Inclusive(a), Inclusive(b)) => a <= t && t <= b,
            (Inclusive(a), Exclusive(b)) => a <= t && t < b,
            (Exclusive(a), Inclusive(b)) => a < t && t <= b,
            (Exclusive(a), Exclusive(b)) => a < t && t < b,
        }
    }
}

pub struct IntervalSet<T>(Vec<Interval<T>>);
