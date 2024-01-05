use ordered_float::{FloatCore, FloatIsNan, NotNan};

pub trait Flip {
    type Flip: Flip<Flip = Self>;
    fn flip(self) -> Self::Flip;
}

pub trait Minimum<T> {
    fn minimum(&self) -> T;
}

pub trait Maximum<T> {
    fn maximum(&self) -> T;
}

pub(crate) trait IntoGeneral {
    type General;
    fn into_general(self) -> Self::General;
}

pub trait Scalar<T>: Ord + Sized {
    type Error;
    fn scalar_try_from(t: T) -> Result<Self, Self::Error>;
    fn scalar_partial_cmp(&self, t: &T) -> Option<std::cmp::Ordering>;

    fn scalar_lt(&self, t: &T) -> bool {
        self.scalar_partial_cmp(t) == Some(std::cmp::Ordering::Less)
    }
    fn scalar_le(&self, t: &T) -> bool {
        self.scalar_partial_cmp(t)
            .map(|o| o != std::cmp::Ordering::Greater)
            .unwrap_or(false)
    }
    fn scalar_gt(&self, t: &T) -> bool {
        self.scalar_partial_cmp(t) == Some(std::cmp::Ordering::Greater)
    }
    fn scalar_ge(&self, t: &T) -> bool {
        self.scalar_partial_cmp(t)
            .map(|o| o != std::cmp::Ordering::Less)
            .unwrap_or(false)
    }
}
impl<T: Ord> Scalar<T> for T {
    type Error = std::convert::Infallible;
    fn scalar_try_from(t: T) -> Result<Self, Self::Error> {
        Ok(t)
    }
    fn scalar_partial_cmp(&self, t: &T) -> Option<std::cmp::Ordering> {
        Some(self.cmp(t))
    }
}
impl<T: FloatCore> Scalar<T> for NotNan<T> {
    type Error = FloatIsNan;
    fn scalar_try_from(t: T) -> Result<Self, Self::Error> {
        NotNan::new(t)
    }
    fn scalar_partial_cmp(&self, t: &T) -> Option<std::cmp::Ordering> {
        NotNan::new(*t).ok().map(|t| self.cmp(&t))
    }
}

pub trait Boundary: Flip + Eq + Copy {
    fn less<S: Scalar<T>, T>(&self, this: &S, t: &T) -> bool;
    fn greater<S: Scalar<T>, T>(&self, this: &S, t: &T) -> bool;
}

pub trait BoundaryOf<LR>: Boundary {
    type Ordered: Ord;
    fn into_ordered(self) -> Self::Ordered;
}
