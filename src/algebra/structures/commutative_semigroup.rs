use super::Semigroup;
#[allow(unused_imports)]
use crate::utils::for_each_tuple; // for cargo-simple-bundler

/// A commutative semigroup.
///
/// # Laws
/// * Commutativity: ∀`x` ∀`y` (`x.op(&y)` = `y.op(&x)`)
pub trait CommutativeSemigroup: Semigroup {}

impl CommutativeSemigroup for () {}

impl<T> CommutativeSemigroup for Option<T> where T: CommutativeSemigroup {}

macro_rules! impl_for_tuple {
  ($($i:tt: $T:ident,)*) => {
    impl<$($T),*> CommutativeSemigroup for ($($T,)*)
    where
      $($T: CommutativeSemigroup,)*
    {
    }
  };
}

for_each_tuple! { impl_for_tuple }
