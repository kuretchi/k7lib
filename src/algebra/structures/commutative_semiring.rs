use super::Semiring;
use crate::num::primitive::Int as PrimInt;
#[allow(unused_imports)]
use crate::utils::for_each_tuple; // for cargo-simple-bundler

/// A commutative semiring.
///
/// # Laws
/// * Commutativity of multiplication: ∀`x` ∀`y` (`x.mul(&y)` = `y.mul(&x)`)
pub trait CommutativeSemiring: Semiring {}

impl CommutativeSemiring for () {}

impl<Int> CommutativeSemiring for Int where Int: PrimInt {}

macro_rules! impl_for_tuple {
  ($($i:tt: $T:ident,)*) => {
    impl<$($T),*> CommutativeSemiring for ($($T,)*)
    where
      $($T: CommutativeSemiring,)*
    {
    }
  };
}

for_each_tuple! { impl_for_tuple }
