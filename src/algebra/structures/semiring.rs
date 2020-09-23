use crate::num::primitive::Int as PrimInt;

/// A semiring.
pub trait Semiring: Clone {
  /// Performs addition.
  fn add(&self, rhs: &Self) -> Self;

  /// Performs multiplication.
  fn mul(&self, rhs: &Self) -> Self;

  /// Returns the additive identity.
  fn zero() -> Self;

  /// Returns the multiplicative identity.
  fn one() -> Self;
}

#[allow(clippy::unused_unit)]
/// The trivial semiring.
impl Semiring for () {
  fn add(&self, _rhs: &Self) -> Self {
    ()
  }
  fn mul(&self, _rhs: &Self) -> Self {
    ()
  }
  fn zero() -> Self {
    ()
  }
  fn one() -> Self {
    ()
  }
}

impl<Int> Semiring for Int
where
  Int: PrimInt,
{
  fn add(&self, rhs: &Self) -> Self {
    *self + *rhs
  }
  fn mul(&self, rhs: &Self) -> Self {
    *self * *rhs
  }
  fn zero() -> Self {
    Int::ZERO
  }
  fn one() -> Self {
    Int::ONE
  }
}
