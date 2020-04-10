use super::Magma;

/// A magma that has an identity element.
///
/// # Laws
/// * ∀`x: T` (`x.op(&T::identity())` = `T::identity().op(&x)` = `x`)
pub trait UnitalMagma: Magma {
  /// Returns an identity element.
  fn identity() -> Self;
}

impl UnitalMagma for () {
  #[allow(clippy::unused_unit)]
  fn identity() -> Self {
    ()
  }
}

impl<T> UnitalMagma for Option<T>
where
  T: Magma,
{
  fn identity() -> Self {
    None
  }
}
