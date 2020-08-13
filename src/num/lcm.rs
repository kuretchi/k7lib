use super::gcd;
use super::primitive::UnsignedInt as PrimUint;

/// Returns the least common multiple if it exists.
///
/// # Examples
///
/// ```
/// # use spella::num::lcm;
/// assert_eq!(lcm::<u32>(18, 12), Some(36));
/// assert_eq!(lcm::<u32>(18, 0), None);
/// ```
pub fn lcm<Int>(x: Int, y: Int) -> Option<Int>
where
  Int: PrimUint,
{
  if x == Int::ZERO || y == Int::ZERO {
    None
  } else {
    Some(x / gcd(x, y) * y)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use quickcheck_macros::quickcheck;
  use std::num::NonZeroU32;

  #[quickcheck]
  fn lcm_with_zero_does_not_exist(x: u32) {
    assert_eq!(lcm(0, x), None);
    assert_eq!(lcm(x, 0), None);
  }

  #[quickcheck]
  fn lcm_prop(x: NonZeroU32, y: NonZeroU32) {
    let x = x.get();
    let y = y.get();
    let m = lcm(x, y).unwrap();
    assert_eq!(m % x, 0);
    assert_eq!(m % y, 0);
    assert!((1..m).all(|z| z % x != 0 || z % y != 0));
  }
}
