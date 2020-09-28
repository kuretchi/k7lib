use super::Semiring;
use crate::num::primitive::SignedInt as PrimSint;

/// A ring.
///
/// # Laws
/// * ∀`x` (`x.add(&x.neg())` = `x.neg().add(&x)` = `Self::zero()`)
pub trait Ring: Semiring {
  /// Returns an additive inverse.
  fn neg(&self) -> Self;
}

#[allow(clippy::unused_unit)]
impl Ring for () {
  fn neg(&self) -> Self {
    ()
  }
}

impl<Int> Ring for Int
where
  Int: PrimSint,
{
  fn neg(&self) -> Self {
    -*self
  }
}
