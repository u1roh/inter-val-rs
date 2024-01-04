#![cfg(test)]
use std::any::{Any, TypeId};

use super::*;
use crate::{Exclusive, Inclusive};

#[test]
fn it_works() {
    let i = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    assert!(i.contains(&0));
    assert!(i.contains(&1));
    assert!(i.contains(&2));
    assert!(!i.contains(&3));
    assert!(!i.contains(&-1));

    // let i = Inclusive(4).to(Inclusive(7)).unwrap();
    // assert!(i.contains(&4));
    // assert!(i.contains(&7));

    // let i = Exclusive(-2).to(Inclusive(5)).unwrap();
    // assert!(!i.contains(&-2));
    // assert!(i.contains(&5));

    let _i = Interval::<NotNan<_>, Inclusive, Inclusive>::not_nan(1.23, 4.56).unwrap();
    let _i = Inclusive
        .not_nan(1.23)
        .unwrap()
        .to(Exclusive.not_nan(4.56).unwrap())
        .unwrap();

    let i = Interval::enclosure_of([3, 9, 2, 5]).unwrap();
    assert_eq!(i.left().val, 2);
    assert_eq!(i.right().val, 9);
}

fn assert_typeid<T: 'static>(a: &dyn Any) {
    assert_eq!(a.type_id(), TypeId::of::<T>());
}

#[test]
fn new_interval() {
    let a: Interval<i32, Inclusive, Exclusive> = Interval::new(0.into(), 3.into()).unwrap();
    assert!(a.contains(&0));
    assert!(a.contains(&1));
    assert!(a.contains(&2));
    assert!(!a.contains(&3));
    assert!(!a.contains(&-1));

    let a = Interval::new(Inclusion::Exclusive.at(0), Inclusion::Exclusive.at(3)).unwrap();
    assert_typeid::<Interval<i32>>(&a);
    assert!(!a.contains(&0));
    assert!(a.contains(&1));
    assert!(!a.contains(&3));

    let a = Interval::<_, Exclusive, Inclusive>::not_nan(1.23, 4.56).unwrap();
    assert!(!a.contains_f(1.23));
    assert!(a.contains_f(1.23000000000001));
    assert!(a.contains_f(4.56));
}

#[test]
fn bound_to_bound() {
    let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    assert_typeid::<Interval<i32, Inclusive, Exclusive>>(&a);

    let a = Inclusive.at(1.23).not_nan_to(Exclusive.at(4.56)).unwrap();
    assert_typeid::<IntervalF<f64, Inclusive, Exclusive>>(&a);
}

#[test]
fn range_into_interval() {
    let a: Interval<_, _, _> = (0..3).try_into().unwrap();
    assert_typeid::<Interval<i32, Inclusive, Exclusive>>(&a);

    let a: Interval<_, _, _> = (0..=3).try_into().unwrap();
    assert_typeid::<Interval<i32, Inclusive, Inclusive>>(&a);

    let a: Interval<_, _, _> = (1.23..4.56).try_into().unwrap();
    assert_typeid::<Interval<NotNan<f64>, Inclusive, Exclusive>>(&a);

    let a: Interval<_, _, _> = (1.23..=4.56).try_into().unwrap();
    assert_typeid::<Interval<NotNan<f64>, Inclusive, Inclusive>>(&a);
}

#[test]
fn ordering() {
    let a: LeftBounded<_, _> = Inclusion::Inclusive.at(0).into();
    let b: LeftBounded<_, _> = Inclusion::Exclusive.at(0).into();
    assert!(a < b);

    let a: RightBounded<_, _> = Inclusion::Inclusive.at(0).into();
    let b: RightBounded<_, _> = Inclusion::Exclusive.at(0).into();
    assert!(a > b);
}
