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

/// The trivial magma.
impl Magma for () {
  #[allow(clippy::unused_unit)]
  fn op(&self, _rhs: &Self) -> Self {
    ()
  }
}

/// Adjoining an identity element `None`.
impl<T> Magma for Option<T>
where
  T: Magma,
{
  fn op(&self, rhs: &Self) -> Self {
    match (self, rhs) {
      (Some(lhs), Some(rhs)) => Some(lhs.op(rhs)),
      (Some(x), None) | (None, Some(x)) => Some(x.clone()),
      (None, None) => None,
    }
  }
}
