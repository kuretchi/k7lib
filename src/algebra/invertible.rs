use algebra::{Magma, Unital};

/// A trait for magma whose all elements have an inverse element.
pub trait Invertible: Magma + Unital {
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
