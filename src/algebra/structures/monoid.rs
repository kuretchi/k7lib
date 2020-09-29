use super::Semigroup;

/// A monoid.
///
/// # Laws
/// * Identity: ∀`x` (`x.op(&Self::identity())` = `Self::identity().op(&x)` = `x`)
pub trait Monoid: Semigroup {
  /// Returns an identity element.
  fn identity() -> Self;
}

#[allow(clippy::unused_unit)]
impl Monoid for () {
  fn identity() -> Self {
    ()
  }
}

impl<T> Monoid for Option<T>
where
  T: Semigroup,
{
  fn identity() -> Self {
    None
  }
}
