# rnmat
Rational number matrix.
```rust
use rnmat::mat::RNMat;

let mut mat = RNMat::new();
mat.push_row(vec![(2, 4), (3, 4)]);
mat.push_row(vec![(5, 6), (-7, 8)]);

assert_eq!(
  mat,
  RNMat::from(
    vec![
      vec![(1, 2), (3, 4)], // 1/2 == 2/4
      vec![(5, 6) (7, -8)], // -7/8 == 7/(-8)
    ]
  )
);

```
