use crate::traits::{Flip, IntoGeneral};
use ordered_float::{FloatCore, FloatIsNan, NotNan};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bound<T, B> {
    pub val: T,
    pub inclusion: B,
}
impl<T: FloatCore, B> Bound<T, B> {
    pub fn into_not_nan(self) -> Result<Bound<NotNan<T>, B>, FloatIsNan> {
        NotNan::new(self.val).map(|val| Bound {
            val,
            inclusion: self.inclusion,
        })
    }
}

impl<T, B: IntoGeneral> IntoGeneral for Bound<T, B> {
    type General = Bound<T, B::General>;
    fn into_general(self) -> Self::General {
        Bound {
            val: self.val,
            inclusion: self.inclusion.into_general(),
        }
    }
}

impl<T, B: Flip> Flip for Bound<T, B> {
    type Flip = Bound<T, B::Flip>;
    fn flip(self) -> Self::Flip {
        Bound {
            val: self.val,
            inclusion: self.inclusion.flip(),
        }
    }
}
