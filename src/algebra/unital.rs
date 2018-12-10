use algebra::Magma;

/// A trait for magma which has an identity element.
pub trait Unital: Magma {
  /// Returns an identity element.
  fn identity() -> Self;
}
