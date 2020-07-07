use super::primitive::UnsignedInt as PrimUint;

use std::cmp;
use std::mem;

/// Returns the greatest common divisor.
///
/// # Examples
///
/// ```
/// # use spella::num::gcd;
/// assert_eq!(gcd::<u32>(18, 12), 6);
/// assert_eq!(gcd::<u32>(18, 0), 18);
/// assert_eq!(gcd::<u32>(0, 0), 0);
/// ```
pub fn gcd<Int>(mut x: Int, mut y: Int) -> Int
where
  Int: PrimUint,
{
  if x == Int::ZERO || y == Int::ZERO {
    return x | y;
  }
  // Stein's binary GCD algorithm
  let k = {
    let x_tz = x.trailing_zeros();
    let y_tz = y.trailing_zeros();
    x >>= x_tz;
    y >>= y_tz;
    cmp::min(x_tz, y_tz)
  };
  while x != y {
    if x < y {
      mem::swap(&mut x, &mut y);
    }
    x -= y;
    x >>= x.trailing_zeros();
  }
  x << k
}

#[cfg(test)]
mod tests {
  use super::*;

  use quickcheck_macros::quickcheck;

  #[test]
  fn gcd_of_zero_and_zero_test() {
    assert_eq!(gcd::<u32>(0, 0), 0);
  }

  #[quickcheck]
  fn zero_is_identity_of_gcd(x: u32) {
    assert_eq!(gcd(0, x), x);
    assert_eq!(gcd(x, 0), x);
  }

  #[quickcheck]
  fn gcd_prop(x: u32, y: u32) {
    let d = gcd(x, y);
    if d == 0 {
      assert!(x == 0 || y == 0);
    } else {
      assert_eq!(x % d, 0);
      assert_eq!(y % d, 0);
      assert!((d + 1..=cmp::min(x, y)).all(|z| x % z != 0 || y % z != 0));
    }
  }
}
