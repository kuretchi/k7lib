use crate::num::{div_ceil, div_floor, primitive::Int as PrimInt};
use std::mem;

/// Returns $\lfloor (x + y) / 2 \rfloor$ if $x \lt y$, $\lceil (x + y) / 2 \rceil$ otherwise.
///
/// # Examples
/// ```
/// # use k7lib::num::midpoint;
/// // floor((2 + 7) / 2) = floor(4.5) = 4
/// assert_eq!(midpoint(2, 7), 4);
///
/// // ceil((7 + 2) / 2) = ceil(4.5) = 5
/// assert_eq!(midpoint(7, 2), 5);
/// ```
/// No overflow occurs:
/// ```
/// # use k7lib::num::midpoint;
/// let max = std::u128::MAX;
/// assert_eq!(midpoint(max, max - 2), max - 1);
/// ```
pub fn midpoint<Int>(mut x: Int, mut y: Int) -> Int
where
  Int: PrimInt,
{
  let div = if x < y { div_floor } else { div_ceil };
  let half = |z| div(z, Int::ONE + Int::ONE);

  if let Some(mid) = x.checked_add(y).map(half) {
    return mid;
  }
  debug_assert!(x < Int::ZERO && y < Int::ZERO || x > Int::ZERO && y > Int::ZERO);
  if x > y {
    mem::swap(&mut x, &mut y);
  }
  x + half(y - x)
}

#[cfg(test)]
mod tests {
  use super::*;
  use quickcheck_macros::quickcheck;
  use std::i32;

  fn naive(x: i8, y: i8) -> i8 {
    let div = if x < y { div_floor } else { div_ceil };
    div(x as i32 + y as i32, 2) as i8
  }

  #[quickcheck]
  fn prop(x: i8, y: i8) {
    assert_eq!(midpoint(x, y), naive(x, y));
  }

  #[test]
  fn zero_max() {
    assert_eq!(midpoint(0, i32::MAX), i32::MAX / 2);
  }

  #[test]
  fn max_zero() {
    assert_eq!(midpoint(i32::MAX, 0), i32::MAX / 2 + 1);
  }

  #[test]
  fn min_zero() {
    assert_eq!(midpoint(i32::MIN, 0), i32::MIN / 2);
  }

  #[test]
  fn zero_min() {
    assert_eq!(midpoint(0, i32::MIN), i32::MIN / 2);
  }

  #[test]
  fn min_max() {
    assert_eq!(midpoint(i32::MIN, i32::MAX), -1);
  }

  #[test]
  fn max_min() {
    assert_eq!(midpoint(i32::MAX, i32::MIN), 0);
  }

  #[test]
  fn nearly_max_floor() {
    assert_eq!(midpoint(i32::MAX - 3, i32::MAX), i32::MAX - 2);
  }

  #[test]
  fn nearly_max_ceil() {
    assert_eq!(midpoint(i32::MAX, i32::MAX - 3), i32::MAX - 1);
  }

  #[test]
  fn nearly_min_floor() {
    assert_eq!(midpoint(i32::MIN, i32::MIN + 3), i32::MIN + 1);
  }

  #[test]
  fn nearly_min_ceil() {
    assert_eq!(midpoint(i32::MIN + 3, i32::MIN), i32::MIN + 2);
  }
}
