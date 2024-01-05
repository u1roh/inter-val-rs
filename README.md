# Mathematical interval library for Rust

Mathematical intervals, i.g., [a, b], (a, b), [a, b), and (a, b].

## Usage
```
use intervals::{Inclusive, Exclusive, Interval};

// Closed interval of i32
let a = Inclusive.at(0).to(Inclusive.at(10)).unwrap();  // [0, 10]
let b = Inclusive.at(5).to(Inclusive.at(15)).unwrap();  // [5, 15]
let c = a.intersection(b).unwrap(); // [0, 10] ∩ [5, 15] = [5, 10]
assert_eq!(c.min(), 5);
assert_eq!(c.max(), 10);

// Half-open interval of f64
let a = Inclusive.at(1.23).float_to(Exclusive.at(4.56)).unwrap();   // [1.23, 4.56)
assert_eq!(a.inf(), 1.23);
assert_eq!(a.sup(), 4.56);
assert!(a.contains(&1.23));
assert!(!a.contains(&4.56));
assert!(a.contains(&(4.56 - 0.000000000000001)));

// Enclosure
let a = Interval::enclosure_of_items(vec![3, 9, 2, 5]).unwrap(); // [2, 9]
assert_eq!(a.min(), 2);
assert_eq!(a.max(), 9);
```