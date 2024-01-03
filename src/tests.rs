#![cfg(test)]
use std::any::{Any, TypeId};

use super::*;
use crate::{Exclusive, Inclusive};

#[test]
fn it_works() {
    let i = Interval::new((0, Inclusive), (3, Exclusive)).unwrap();
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
    let _i = Interval::not_nan((1.23, Inclusive), (4.56, Exclusive)).unwrap();

    let i = Interval::enclose([3, 9, 2, 5]).unwrap();
    assert_eq!(i.lower().val, 2);
    assert_eq!(i.upper().val, 9);
}

fn assert_typeid<T: 'static>(a: &dyn Any) {
    assert_eq!(a.type_id(), TypeId::of::<T>());
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
