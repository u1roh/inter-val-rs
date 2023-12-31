use crate::*;

impl<T> From<Inclusive<T>> for Boundary<T> {
    fn from(b: Inclusive<T>) -> Self {
        Self::Inclusive(b.0)
    }
}
impl<T> From<Exclusive<T>> for Boundary<T> {
    fn from(b: Exclusive<T>) -> Self {
        Self::Exclusive(b.0)
    }
}

impl<T: ordered_float::FloatCore> TryFrom<Inclusive<T>> for Inclusive<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Inclusive<T>) -> Result<Self, Self::Error> {
        NotNan::new(b.0).map(Self)
    }
}
impl<T: ordered_float::FloatCore> TryFrom<Exclusive<T>> for Exclusive<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Exclusive<T>) -> Result<Self, Self::Error> {
        NotNan::new(b.0).map(Self)
    }
}
impl<T: ordered_float::FloatCore> TryFrom<Boundary<T>> for Boundary<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Boundary<T>) -> Result<Self, Self::Error> {
        match b {
            Boundary::Inclusive(t) => NotNan::new(t).map(Self::Inclusive),
            Boundary::Exclusive(t) => NotNan::new(t).map(Self::Exclusive),
        }
    }
}
impl<T: ordered_float::FloatCore> TryFrom<Inclusive<T>> for Boundary<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Inclusive<T>) -> Result<Self, Self::Error> {
        Boundary::from(b).try_into()
    }
}
impl<T: ordered_float::FloatCore> TryFrom<Exclusive<T>> for Boundary<NotNan<T>> {
    type Error = ordered_float::FloatIsNan;
    fn try_from(b: Exclusive<T>) -> Result<Self, Self::Error> {
        Boundary::from(b).try_into()
    }
}
