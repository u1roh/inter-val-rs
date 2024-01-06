use crate::traits::{Flip, IntoGeneral};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bound<T, B> {
    pub limit: T,
    pub bounding: B,
}

impl<T, B: IntoGeneral> IntoGeneral for Bound<T, B> {
    type General = Bound<T, B::General>;
    fn into_general(self) -> Self::General {
        Bound {
            limit: self.limit,
            bounding: self.bounding.into_general(),
        }
    }
}

impl<T, B: Flip> Flip for Bound<T, B> {
    type Flip = Bound<T, B::Flip>;
    fn flip(self) -> Self::Flip {
        Bound {
            limit: self.limit,
            bounding: self.bounding.flip(),
        }
    }
}
