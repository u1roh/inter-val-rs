use crate::{
    traits::{Ceil, Flip, Floor, IntoGeneral},
    BoundType, Exclusive, Inclusive,
};

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

macro_rules! impl_ceil_floor_for_integer {
    ($T:ty) => {
        impl Ceil<$T> for Bound<$T, Inclusive> {
            fn ceil(&self) -> $T {
                self.limit
            }
        }
        impl Ceil<$T> for Bound<$T, Exclusive> {
            fn ceil(&self) -> $T {
                self.limit + 1
            }
        }
        impl Ceil<$T> for Bound<$T, BoundType> {
            fn ceil(&self) -> $T {
                match self.bound_type {
                    BoundType::Inclusive => self.limit,
                    BoundType::Exclusive => self.limit + 1,
                }
            }
        }
        impl Floor<$T> for Bound<$T, Inclusive> {
            fn floor(&self) -> $T {
                self.limit
            }
        }
        impl Floor<$T> for Bound<$T, Exclusive> {
            fn floor(&self) -> $T {
                self.limit - 1
            }
        }
        impl Floor<$T> for Bound<$T, BoundType> {
            fn floor(&self) -> $T {
                match self.bound_type {
                    BoundType::Inclusive => self.limit,
                    BoundType::Exclusive => self.limit - 1,
                }
            }
        }
    };
}
impl_ceil_floor_for_integer!(i8);
impl_ceil_floor_for_integer!(i16);
impl_ceil_floor_for_integer!(i32);
impl_ceil_floor_for_integer!(i64);
impl_ceil_floor_for_integer!(i128);
impl_ceil_floor_for_integer!(isize);

fn ceil_exclusive<T: num::Float>(t: T) -> T {
    let ceil = t.ceil();
    if ceil == t {
        ceil + T::one()
    } else {
        ceil
    }
}

fn floor_exclusive<T: num::Float>(t: T) -> T {
    let ceil = t.floor();
    if ceil == t {
        ceil - T::one()
    } else {
        ceil
    }
}

macro_rules! impl_ceil_floor_for_float {
    ($T:ty) => {
        impl Ceil<$T> for Bound<$T, Inclusive> {
            fn ceil(&self) -> $T {
                self.limit.ceil()
            }
        }
        impl Ceil<$T> for Bound<$T, Exclusive> {
            fn ceil(&self) -> $T {
                ceil_exclusive(self.limit)
            }
        }
        impl Ceil<$T> for Bound<$T, BoundType> {
            fn ceil(&self) -> $T {
                match self.bound_type {
                    BoundType::Inclusive => self.limit.ceil(),
                    BoundType::Exclusive => ceil_exclusive(self.limit),
                }
            }
        }
        impl Floor<$T> for Bound<$T, Inclusive> {
            fn floor(&self) -> $T {
                self.limit.floor()
            }
        }
        impl Floor<$T> for Bound<$T, Exclusive> {
            fn floor(&self) -> $T {
                floor_exclusive(self.limit)
            }
        }
        impl Floor<$T> for Bound<$T, BoundType> {
            fn floor(&self) -> $T {
                match self.bound_type {
                    BoundType::Inclusive => self.limit.floor(),
                    BoundType::Exclusive => floor_exclusive(self.limit),
                }
            }
        }
    };
}
impl_ceil_floor_for_float!(f32);
impl_ceil_floor_for_float!(f64);
