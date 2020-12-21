use crate::algebra::structures::{CommutativeSemigroup, Group, Monoid, Ring, Semigroup, Semiring};

/// A commutative monoid under semiring addition.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Sum<T>(pub T);

impl<T> Semigroup for Sum<T>
where
  T: Semiring,
{
  fn op(&self, rhs: &Self) -> Self {
    Self(self.0.add(&rhs.0))
  }
}

impl<T> CommutativeSemigroup for Sum<T> where T: Semiring {}

impl<T> Monoid for Sum<T>
where
  T: Semiring,
{
  fn identity() -> Self {
    Self(T::zero())
  }
}

impl<T> Group for Sum<T>
where
  T: Ring,
{
  fn invert(&self) -> Self {
    Self(self.0.neg())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Sum::identity().op(&Sum(3)).op(&Sum(1).invert()).op(&Sum(4)), Sum(3 - 1 + 4));
  }
}
