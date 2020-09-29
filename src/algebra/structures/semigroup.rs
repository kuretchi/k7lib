/// A semigroup.
///
/// # Laws
/// * Associativity: ∀`x` ∀`y` ∀`z` (`x.op(&y).op(&z)` = `x.op(&y.op(&z))`)
pub trait Semigroup: Clone {
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

/// The trivial semigroup.
#[allow(clippy::unused_unit)]
impl Semigroup for () {
  fn op(&self, _rhs: &Self) -> Self {
    ()
  }
}

/// Adjoining an identity element `None`.
impl<T> Semigroup for Option<T>
where
  T: Semigroup,
{
  fn op(&self, rhs: &Self) -> Self {
    match (self, rhs) {
      (Some(lhs), Some(rhs)) => Some(lhs.op(rhs)),
      (Some(x), None) | (None, Some(x)) => Some(x.clone()),
      (None, None) => None,
    }
  }
}
