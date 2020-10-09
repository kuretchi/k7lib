use crate::algebra::structures::{CommutativeSemigroup, Monoid, Semigroup};
use crate::cmp::Bounded;

use std::cmp;

/// A monoid that returns the maximum value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Max<T>(pub T);

impl<T> Semigroup for Max<T>
where
  T: Clone + Ord,
{
  fn op(&self, rhs: &Self) -> Self {
    Max(cmp::max(&self.0, &rhs.0).clone())
  }
}

impl<T> CommutativeSemigroup for Max<T> where T: Clone + Ord {}

impl<T> Monoid for Max<T>
where
  T: Clone + Ord + Bounded,
{
  fn identity() -> Self {
    Max(T::MIN)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    let iter = || vec.iter().copied();

    assert_eq!(
      iter().max().unwrap(),
      iter().fold(Max::identity(), |acc, x| acc.op(&Max(x))).0
    );
  }
}
