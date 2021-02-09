use super::Monoid;
#[allow(unused_imports)]
use crate::utils::for_each_tuple; // for cargo-simple-bundler

/// A group.
///
/// # Laws
/// * ∀`x` (`x.op(&x.invert())` = `x.invert().op(&x)` = `Self::identity()`)
pub trait Group: Monoid {
  /// Returns an inverse element.
  fn invert(&self) -> Self;

  /// Returns `self.op(&rhs.invert())`.
  fn inverse_op(&self, rhs: &Self) -> Self {
    self.op(&rhs.invert())
  }

  /// Assigns `self.inverse_op(rhs)` to `self`.
  fn inverse_op_assign_right(&mut self, rhs: &Self) {
    *self = self.inverse_op(rhs);
  }

  /// Assigns `lhs.inverse_op(self)` to `self`.
  fn inverse_op_assign_left(&mut self, lhs: &Self) {
    *self = lhs.inverse_op(self);
  }
}

#[allow(clippy::unused_unit)]
impl Group for () {
  fn invert(&self) -> Self {
    ()
  }
}

macro_rules! impl_for_tuple {
  ($($i:tt: $T:ident,)*) => {
    impl<$($T),*> Group for ($($T,)*)
    where
      $($T: Group,)*
    {
      fn invert(&self) -> Self {
        ($(self.$i.invert(),)*)
      }
    }
  };
}

for_each_tuple! { impl_for_tuple }
