# Intervals on ℝⁿ

Mathematical intervals, i.g., [a, b], (a, b), [a, b), and (a, b] on ℝ (real number line).
Also supports multi-dimensional axis-aligned boxes.

NOTE: Not yet stable.

## Intervals on ℝ
Intervals like *[a, b]*, *(a, b)*, *[a, b)*, and *(a, b]* for any `PartialOrd` type.

### Properties
```txt
lower_bound     left              . center          right    upper_bound
...------------>|<------- self -------------------->|<------------ ...
                inf                                 sup
                [<------------ closure ------------>]
                 (<----------- interior ---------->)
```

### Set operations
```txt
|<------------- a ----------------->|   . p           |<-------- c -------->|
       |<--------------- b ------------------->|
       |<--- a.intersection(&b) --->|
                                    |<-- a.gap(&c) -->|
|<------------- a.hull(p) ------------->|
|<---------------------------------- a.span(&c) --------------------------->|
|<--------------------------------->|        +        |<------------------->| a.union(&c)
|<---->| a.difference(&b)
                                               |<- δ -+---- c.dilate(δ) ----+- δ ->|
```

### Examples
```rust
use inter_val::{Inclusive, Exclusive, Interval};

// Closed interval of i32
let a = Inclusive.at(0).to(Inclusive.at(10));  // [0, 10]
assert!(a.contains(&3));

// Half-open interval of f64
let b = Inclusive.at(1.23).to(Exclusive.at(4.56));   // [1.23, 4.56)
assert!(!b.contains(&4.56));
assert!(b.contains(&(4.56 - 0.000000000000001)));

// Intersection
let c = Inclusive.between(5, 15);  // [5, 15]
let isect = a.intersection(&c).unwrap(); // [0, 10] ∩ [5, 15] = [5, 10]
assert_eq!(isect.inf(), &5);
assert_eq!(isect.sup(), &10);

// Span & Gap
let d = Inclusive.between(12, 15);  // [12, 15]
let span = a.span(&d);  // [0, 15]
let gap = a.gap(&d);    // (10, 12)
assert_eq!(span, Inclusive.between(0, 15));
assert_eq!(gap.unwrap(), Exclusive.between(10, 12));

// Union
let union = a.union(&d);
assert_eq!(union.span, span);
assert_eq!(union.gap, gap);
assert_eq!(union.into_vec(), vec![a, d]);

// Hull
let a = Interval::<_>::hull_many(vec![3, 9, 2, 5]).unwrap(); // [2, 9]
assert_eq!(a, Inclusive.between(2, 9));
```

## Axis-aligned box on ℝⁿ
Boxes represented by Cartesian product of intervals.
```rust
use inter_val::{Box2, Inclusive};

// [0.0, 10.0] × [5.0, 20.0]
let a: Box2<f64> = Box2::new(Inclusive.between(0.0, 10.0), Inclusive.between(5.0, 20.0));

// Another way to construct [0.0, 10.0] × [5.0, 20.0]
let b: Box2<f64> = Box2::between(&[0.0, 5.0], &[10.0, 20.0]);
assert_eq!(a, b);

// Hull
let b = a.hull(&[12.3, 7.5]);
assert_eq!(b, Box2::between(&[0.0, 5.0], &[12.3, 20.0]));
```

## Future work
* Enhance `BoxN`.
* Interval set.
* Sufficient tests.

Not promised :-)

