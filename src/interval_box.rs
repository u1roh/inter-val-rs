use crate::bound_type::{Left, Right};
use crate::ndim::NDim;
use crate::traits::BoundaryOf;
use crate::{Bound, Exclusive, Inclusive, Interval};

pub trait Point<const N: usize, T>:
    From<[T; N]> + Into<[T; N]> + std::ops::Index<usize, Output = T>
{
    fn iter(&self) -> std::slice::Iter<T>;
}

impl<const N: usize, T> Point<N, T> for [T; N] {
    fn iter(&self) -> std::slice::Iter<T> {
        (self as &[T]).iter()
    }
}

impl<const N: usize, T> Point<N, T> for NDim<N, T> {
    fn iter(&self) -> std::slice::Iter<T> {
        self.iter()
    }
}

#[cfg(feature = "nalgebra")]
impl<const N: usize, T: Clone + std::fmt::Debug + PartialEq + 'static> Point<N, T>
    for nalgebra::Point<T, N>
{
    fn iter(&self) -> std::slice::Iter<T> {
        self.coords.as_slice().iter()
    }
}

/// n-dimensional axis-aligned box as a cartesian product set of intervals, i.g., *[a, b)^n*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxN<const N: usize, T, L = Inclusive, R = L>(NDim<N, Interval<T, L, R>>);

impl<const N: usize, T, L, R> std::ops::Deref for BoxN<N, T, L, R> {
    type Target = NDim<N, Interval<T, L, R>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize, T, L, R> std::ops::DerefMut for BoxN<N, T, L, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize, T, L, R> From<[Interval<T, L, R>; N]> for BoxN<N, T, L, R> {
    fn from(src: [Interval<T, L, R>; N]) -> Self {
        Self(src.into())
    }
}

impl<const N: usize, T, L, R> From<BoxN<N, T, L, R>> for [Interval<T, L, R>; N] {
    fn from(src: BoxN<N, T, L, R>) -> Self {
        src.0.into_array()
    }
}

impl<const N: usize, T, L, R> AsRef<[Interval<T, L, R>; N]> for BoxN<N, T, L, R> {
    fn as_ref(&self) -> &[Interval<T, L, R>; N] {
        self.0.as_array()
    }
}

impl<const N: usize, T, L, R> BoxN<N, T, L, R> {
    pub fn from_array(src: [Interval<T, L, R>; N]) -> Self {
        src.into()
    }
    pub fn into_array(self) -> [Interval<T, L, R>; N] {
        self.into()
    }
}

impl<T, L, R> BoxN<2, T, L, R> {
    pub fn new(x: Interval<T, L, R>, y: Interval<T, L, R>) -> Self {
        Self([x, y].into())
    }
}
impl<T, L, R> BoxN<3, T, L, R> {
    pub fn new(x: Interval<T, L, R>, y: Interval<T, L, R>, z: Interval<T, L, R>) -> Self {
        Self([x, y, z].into())
    }
}
impl<T, L, R> BoxN<4, T, L, R> {
    pub fn new(
        x: Interval<T, L, R>,
        y: Interval<T, L, R>,
        z: Interval<T, L, R>,
        w: Interval<T, L, R>,
    ) -> Self {
        Self([x, y, z, w].into())
    }
}

impl<const N: usize, T: PartialOrd + Clone, L: BoundaryOf<Left>, R: BoundaryOf<Right>>
    BoxN<N, T, L, R>
{
    /// ```
    /// use inter_val::{Box2, Exclusive};
    /// let a: Box2<i32> = Box2::try_between(&[0, 0], &[10, 20]).unwrap();
    /// assert_eq!(a.x.inf(), &0);
    /// assert_eq!(a.y.sup(), &20);
    ///
    /// assert!(Box2::<i32, Exclusive>::try_between(&[0, 0], &[0, 1]).is_none());
    /// ```
    pub fn try_between<P: Point<N, T>>(a: &P, b: &P) -> Option<Self>
    where
        T: Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        let mut tmp: [_; N] =
            std::array::from_fn(|i| Interval::try_between(a[i].clone(), b[i].clone()));
        tmp.iter()
            .all(|i| i.is_some())
            .then(|| std::array::from_fn(|i| tmp[i].take().unwrap()).into())
    }

    /// ```
    /// use inter_val::Box2;
    /// let a: Box2<i32> = Box2::between(&[0, 0], &[10, 20]);
    /// assert_eq!(a.inf(), [0, 0]);
    /// assert_eq!(a.sup(), [10, 20]);
    pub fn between<P: Point<N, T>>(a: &P, b: &P) -> Self
    where
        T: Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        std::array::from_fn(|i| Interval::between(a[i].clone(), b[i].clone())).into()
    }

    pub fn inf(&self) -> NDim<N, T> {
        std::array::from_fn(|i| self[i].inf().clone()).into()
    }

    pub fn sup(&self) -> NDim<N, T> {
        std::array::from_fn(|i| self[i].sup().clone()).into()
    }

    pub fn inf_point<P: Point<N, T>>(&self) -> P {
        std::array::from_fn(|i| self[i].inf().clone()).into()
    }

    pub fn sup_point<P: Point<N, T>>(&self) -> P {
        std::array::from_fn(|i| self[i].sup().clone()).into()
    }

    #[cfg(feature = "nalgebra")]
    pub fn inf_nalgebra(&self) -> nalgebra::Point<T, N>
    where
        T: Clone + std::fmt::Debug + PartialEq + 'static,
    {
        self.inf_point()
    }

    #[cfg(feature = "nalgebra")]
    pub fn sup_nalgebra(&self) -> nalgebra::Point<T, N>
    where
        T: Clone + std::fmt::Debug + PartialEq + 'static,
    {
        self.sup_point()
    }

    pub fn contains<P: Point<N, T>>(&self, t: &P) -> bool {
        self.iter().zip(t.iter()).all(|(i, t)| i.contains(t))
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(i, o)| i.includes(o))
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(i, j)| i.overlaps(j))
    }

    pub fn closure(&self) -> BoxN<N, T, Inclusive> {
        std::array::from_fn(|i| self[i].clone().closure()).into()
    }

    pub fn interior(&self) -> Option<BoxN<N, T, Exclusive>> {
        let mut tmp: [_; N] = std::array::from_fn(|i| self[i].clone().interior());
        tmp.iter()
            .all(|i| i.is_some())
            .then(|| std::array::from_fn(|i| tmp[i].take().unwrap()).into())
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let mut tmp: [_; N] = std::array::from_fn(|i| self[i].intersection(&other[i]));
        tmp.iter()
            .all(|i| i.is_some())
            .then(|| std::array::from_fn(|i| tmp[i].take().unwrap()).into())
    }

    pub fn span(&self, other: &Self) -> Self {
        std::array::from_fn(|i| self[i].clone().span(&other[i])).into()
    }

    pub fn dilate(&self, delta: T) -> Self
    where
        T: std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
    {
        std::array::from_fn(|i| self[i].clone().dilate(delta.clone())).into()
    }

    /// ```
    /// use inter_val::Box2;
    /// let a: Box2<i32> = Box2::between(&[0, 0], &[10, 10]);
    /// let b = a.hull(&[20, 5]);
    /// assert_eq!(b, Box2::between(&[0, 0], &[20, 10]));
    /// ```
    pub fn hull<P: Point<N, T>>(self, p: &P) -> Self {
        std::array::from_fn(|i| self[i].clone().hull(p[i].clone())).into()
    }

    pub fn span_many<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.span(&item.into())))
    }

    pub fn hull_many<'a>(items: impl IntoIterator<Item = &'a [T; N]>) -> Option<Self>
    where
        T: Clone + Into<Bound<T, L>> + Into<Bound<T, R>> + 'a,
    {
        let mut items = items.into_iter();
        let mut lower = items.next()?.clone();
        let mut upper = lower.clone();
        for p in items {
            for i in 0..N {
                if p[i] < lower[i] {
                    lower[i] = p[i].clone();
                } else if upper[i] < p[i] {
                    upper[i] = p[i].clone();
                }
            }
        }
        Self::try_between(&lower, &upper)
    }
}

impl<const N: usize, T, L, R> BoxN<N, T, L, R>
where
    T: PartialOrd + Clone + num::Num,
    L: BoundaryOf<Left>,
    R: BoundaryOf<Right>,
{
    pub fn size(&self) -> NDim<N, T> {
        std::array::from_fn(|i| self[i].measure()).into()
    }
    pub fn measure(&self) -> T {
        self.iter()
            .map(|item| item.measure())
            .fold(T::one(), |a, b| a * b)
    }
}

impl<const N: usize, T: num::Float, L: BoundaryOf<Left>, R: BoundaryOf<Right>> BoxN<N, T, L, R> {
    pub fn center(&self) -> NDim<N, T> {
        std::array::from_fn(|i| self[i].center()).into()
    }

    /// IoU - Intersection over Union.
    pub fn iou(&self, other: &Self) -> T {
        self.intersection(other)
            .map(|intersection| {
                let m = intersection.measure();
                m / (self.measure() + other.measure() - m)
            })
            .unwrap_or(T::zero())
    }
}

#[cfg(feature = "nalgebra")]
#[test]
fn test_nalgebra() {
    use nalgebra as na;
    let p1 = na::Point2::new(0, 0);
    let p2 = na::Point2::new(10, 20);
    let b = BoxN::<2, i32>::try_between(&p1, &p2).unwrap();
    assert_eq!(b.inf_nalgebra(), p1);
    assert_eq!(b.sup_nalgebra(), p2);

    let p = na::Point2::new(5, 15);
    assert!(b.contains(&p));
}
