use crate::algebra::structures::{
  AssociativeMagma, CommutativeMagma, InvertibleMagma, Magma, Ring, Semiring, UnitalMagma,
};

/// A commutative monoid under semiring addition.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Sum<T>(pub T);

impl<T> Magma for Sum<T>
where
  T: Semiring,
{
  fn op(&self, rhs: &Self) -> Self {
    Self(self.0.add(&rhs.0))
  }
}

impl<T> AssociativeMagma for Sum<T> where T: Semiring {}

impl<T> CommutativeMagma for Sum<T> where T: Semiring {}

impl<T> UnitalMagma for Sum<T>
where
  T: Semiring,
{
  fn identity() -> Self {
    Self(T::zero())
  }
}

impl<T> InvertibleMagma for Sum<T>
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
    assert_eq!(
      Sum::identity().op(&Sum(3)).op(&Sum(1).invert()).op(&Sum(4)),
      Sum(3 - 1 + 4)
    );
  }
}
