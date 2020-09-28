use crate::algebra::structures::{
  AssociativeMagma, CommutativeMagma, Magma, Semiring, UnitalMagma,
};

/// A monoid under semiring multiplication.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Product<T>(pub T);

impl<T> Magma for Product<T>
where
  T: Semiring,
{
  fn op(&self, rhs: &Self) -> Self {
    Self(self.0.mul(&rhs.0))
  }
}

impl<T> AssociativeMagma for Product<T> where T: Semiring {}

impl<T> CommutativeMagma for Product<T> where T: Semiring {}

impl<T> UnitalMagma for Product<T>
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
      Product::identity()
        .op(&Product(3))
        .op(&Product(-1))
        .op(&Product(4)),
      Product(3 * -1 * 4)
    );
  }
}
