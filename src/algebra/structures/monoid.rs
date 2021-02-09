use super::Semigroup;
#[allow(unused_imports)]
use crate::utils::for_each_tuple; // for cargo-simple-bundler

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

macro_rules! impl_for_tuple {
  ($($i:tt: $T:ident,)*) => {
    impl<$($T),*> Monoid for ($($T,)*)
    where
      $($T: Monoid,)*
    {
      fn identity() -> Self {
        ($(<$T>::identity(),)*)
      }
    }
  };
}

for_each_tuple! { impl_for_tuple }
