#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bound<T> {
    Inclusive(T),
    Exclusive(T),
}
impl<T> From<T> for Bound<T> {
    fn from(t: T) -> Self {
        Bound::Inclusive(t) // default to inclusive
    }
}
impl<T> Bound<T> {
    pub fn into_value(self) -> T {
        match self {
            Bound::Inclusive(t) => t,
            Bound::Exclusive(t) => t,
        }
    }
}

pub struct Interval<T> {
    lower: Bound<T>,
    upper: Bound<T>,
}

pub struct IntervalSet<T>(Vec<Interval<T>>);

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
