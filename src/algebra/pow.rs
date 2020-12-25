use crate::{algebra::structures::Monoid, num::primitive::UnsignedInt as PrimUint};

/// Raises `x` to the power of `n`, using exponentiation by squaring.
///
/// # Time complexity
/// $\Theta(\log(n))$ where $n$ = `n`
pub fn pow<M, Int>(mut x: M, mut n: Int) -> M
where
  M: Monoid,
  Int: PrimUint,
{
  // Exponentiation by squaring
  let mut acc = M::identity();
  while n != Int::ZERO {
    if n % (Int::ONE + Int::ONE) != Int::ZERO {
      // x^n = x * x^(n - 1)
      acc = acc.op(&x);
    }
    // x^n = (x^2)^(n / 2)
    x = x.op(&x);
    n /= Int::ONE + Int::ONE;
  }
  acc
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::algebra::structures::Concat;
  use quickcheck_macros::quickcheck;

  #[quickcheck]
  fn prop(s: Vec<u8>, n: u8) {
    let t = std::iter::repeat(s.iter().copied()).take(n.into()).flatten().collect::<Vec<_>>();
    assert_eq!(pow(Concat(s), n).0, t);
  }

  #[quickcheck]
  fn pow0_prop(s: Vec<u8>) {
    assert!(pow(Concat(s), 0u32).0.is_empty());
  }

  #[quickcheck]
  fn pow1_prop(s: Vec<u8>) {
    let t = s.clone();
    assert_eq!(pow(Concat(s), 1u32).0, t);
  }
}
