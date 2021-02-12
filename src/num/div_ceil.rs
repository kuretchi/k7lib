use crate::num::primitive::Int as PrimInt;

/// Returns $\lceil x / y \rceil$, the smallest integer greater than or equal to $x / y$.
///
/// # Examples
/// ```
/// # use k7lib::num::div_ceil;
/// // 3 / 2 = 1.5, so it returns 2.
/// assert_eq!(div_ceil(3, 2), 2);
/// ```
pub fn div_ceil<Int>(x: Int, y: Int) -> Int
where
  Int: PrimInt,
{
  let q = x / y;
  if (x >= Int::ZERO) == (y >= Int::ZERO) && x % y != Int::ZERO {
    q + Int::ONE
  } else {
    q
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn zero_positive() {
    assert_eq!(div_ceil(0, 2), 0);
  }

  #[test]
  fn zero_negative() {
    assert_eq!(div_ceil(0, -2), 0);
  }

  #[test]
  fn positive_positive_divisible() {
    assert_eq!(div_ceil(10, 2), 5);
  }

  #[test]
  fn negative_negative_divisible() {
    assert_eq!(div_ceil(-10, -2), 5);
  }

  #[test]
  fn positive_negative_divisible() {
    assert_eq!(div_ceil(10, -2), -5);
  }

  #[test]
  fn negative_positive_divisible() {
    assert_eq!(div_ceil(-10, 2), -5);
  }

  #[test]
  fn positive_positive_indivisible() {
    assert_eq!(div_ceil(10, 3), 4);
  }

  #[test]
  fn negative_negative_indivisible() {
    assert_eq!(div_ceil(-10, -3), 4);
  }

  #[test]
  fn positive_negative_indivisible() {
    assert_eq!(div_ceil(10, -3), -3);
  }

  #[test]
  fn negative_positive_indivisible() {
    assert_eq!(div_ceil(-10, 3), -3);
  }
}
