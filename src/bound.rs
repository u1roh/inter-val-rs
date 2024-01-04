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
