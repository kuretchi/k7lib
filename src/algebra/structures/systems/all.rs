use crate::algebra::structures::{AssociativeMagma, CommutativeMagma, Magma, UnitalMagma};

/// A monoid under conjunction `&&`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct All(pub bool);

impl Magma for All {
  fn op(&self, rhs: &Self) -> Self {
    All(self.0 && rhs.0)
  }
}

impl AssociativeMagma for All {}

impl CommutativeMagma for All {}

impl UnitalMagma for All {
  fn identity() -> Self {
    All(true)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let vec = vec![true, true, false, true, false];
    let iter = || vec.iter().copied();

    assert_eq!(
      iter().all(|x| x),
      iter().fold(All::identity(), |acc, x| acc.op(&All(x))).0
    );
  }

  #[test]
  fn test_all_true() {
    let vec = vec![true, true, true, true, true];
    let iter = || vec.iter().copied();

    assert_eq!(
      iter().all(|x| x),
      iter().fold(All::identity(), |acc, x| acc.op(&All(x))).0
    );
  }
}
