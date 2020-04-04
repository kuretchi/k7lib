/// A magma.
pub trait Magma: Clone {
  /// Performs a binary operation.
  fn op(&self, rhs: &Self) -> Self;

  /// Assigns `self.op(rhs)` to `self`.
  fn op_assign_right(&mut self, rhs: &Self) {
    *self = self.op(rhs);
  }

  /// Assigns `lhs.op(self)` to `self`.
  fn op_assign_left(&mut self, lhs: &Self) {
    *self = lhs.op(self);
  }
}
