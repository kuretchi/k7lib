use crate::algebra::structures::{
  CommutativeSemigroup, CommutativeSemiring, Monoid, Semigroup, Semiring,
};

/// A monoid under semiring multiplication.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Product<T>(pub T);

impl<T> Semigroup for Product<T>
where
  T: Semiring,
{
  fn op(&self, rhs: &Self) -> Self {
    Self(self.0.mul(&rhs.0))
  }
}

impl<T> CommutativeSemigroup for Product<T> where T: CommutativeSemiring {}

impl<T> Monoid for Product<T>
where
  T: Semiring,
{
  fn identity() -> Self {
    Self(T::one())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Product::identity().op(&Product(3)).op(&Product(-1)).op(&Product(4)),
      Product(3 * -1 * 4)
    );
  }
}
