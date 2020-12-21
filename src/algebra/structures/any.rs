use crate::algebra::structures::{CommutativeSemigroup, Monoid, Semigroup};

/// A monoid under disjunction `||`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Any(pub bool);

impl Semigroup for Any {
  fn op(&self, rhs: &Self) -> Self {
    Any(self.0 || rhs.0)
  }
}

impl CommutativeSemigroup for Any {}

impl Monoid for Any {
  fn identity() -> Self {
    Any(false)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let vec = vec![true, true, false, true, false];
    let iter = || vec.iter().copied();

    assert_eq!(iter().any(|x| x), iter().fold(Any::identity(), |acc, x| acc.op(&Any(x))).0);
  }

  #[test]
  fn test_all_false() {
    let vec = vec![false, false, false, false, false];
    let iter = || vec.iter().copied();

    assert_eq!(iter().any(|x| x), iter().fold(Any::identity(), |acc, x| acc.op(&Any(x))).0);
  }
}
