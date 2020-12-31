use super::Semiring;
use crate::num::primitive::SignedInt as PrimSint;
#[allow(unused_imports)]
use crate::utils::for_each_tuple; // for cargo-simple-bundler

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

macro_rules! impl_for_tuple {
  ($($i:tt: $T:ident,)*) => {
    impl<$($T),*> Ring for ($($T,)*)
    where
      $($T: Ring,)*
    {
      fn neg(&self) -> Self {
        ($(self.$i.neg(),)*)
      }
    }
  };
}

for_each_tuple! { impl_for_tuple }
