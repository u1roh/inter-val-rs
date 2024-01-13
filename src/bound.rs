use crate::traits::{Flip, IntoGeneral};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bound<T, B> {
    pub limit: T,
    pub bound_type: B,
}

impl<T, B: IntoGeneral> IntoGeneral for Bound<T, B> {
    type General = Bound<T, B::General>;
    fn into_general(self) -> Self::General {
        Bound {
            limit: self.limit,
            bound_type: self.bound_type.into_general(),
        }
    }
}

impl<T, B: Flip> Flip for Bound<T, B> {
    type Flip = Bound<T, B::Flip>;
    fn flip(self) -> Self::Flip {
        Bound {
            limit: self.limit,
            bound_type: self.bound_type.flip(),
        }
    }
}

impl<T, B> Bound<T, B> {
    pub fn cast<U: From<T>>(self) -> Bound<U, B> {
        Bound {
            limit: self.limit.into(),
            bound_type: self.bound_type,
        }
    }
}
impl<T: num::NumCast, B> Bound<T, B> {
    pub fn try_cast<U: num::NumCast>(self) -> Option<Bound<U, B>> {
        Some(Bound {
            limit: num::cast(self.limit)?,
            bound_type: self.bound_type,
        })
    }
}
