use algebra::Magma;

/// A trait for magma which has an identity element.
pub trait UnitalMagma: Magma {
  /// Returns an identity element.
  fn identity() -> Self;
}
