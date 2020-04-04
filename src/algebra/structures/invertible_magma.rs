use super::{Magma, UnitalMagma};

/// A magma whose all elements have an inverse element.
///
/// # Laws
/// * ∀`x: T` (`x.op(&x.invert())` = `x.invert().op(&x)` = `T::identity()`)
pub trait InvertibleMagma: Magma + UnitalMagma {
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
