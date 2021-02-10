use super::gcd;
use super::primitive::UnsignedInt as PrimUint;

/// Returns the least common multiple.
///
/// # Examples
///
/// ```
/// # use k7lib::num::lcm;
/// assert_eq!(lcm::<u32>(18, 12), 36);
/// assert_eq!(lcm::<u32>(18, 0), 0);
/// assert_eq!(lcm::<u32>(0, 0), 0);
/// ```
pub fn lcm<Int>(x: Int, y: Int) -> Int
where
  Int: PrimUint,
{
  if x == Int::ZERO || y == Int::ZERO {
    Int::ZERO
  } else {
    x / gcd(x, y) * y
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use quickcheck_macros::quickcheck;
  use std::num::NonZeroU32;

  #[quickcheck]
  fn lcm_with_zero(x: u32) {
    assert_eq!(lcm(0, x), 0);
    assert_eq!(lcm(x, 0), 0);
  }

  #[quickcheck]
  fn lcm_prop(x: NonZeroU32, y: NonZeroU32) {
    let x = x.get();
    let y = y.get();
    let m = lcm(x, y);
    assert_eq!(m % x, 0);
    assert_eq!(m % y, 0);
    assert!((1..m).all(|z| z % x != 0 || z % y != 0));
  }
}
