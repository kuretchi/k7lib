use crate::algebra::Magma;

/// A magma that has an identity element.
///
/// # Laws
/// * ∀`x: T` (`x.op(&T::identity())` = `T::identity().op(&x)` = `x`)
pub trait UnitalMagma: Magma {
  /// Returns an identity element.
  fn identity() -> Self;
}
