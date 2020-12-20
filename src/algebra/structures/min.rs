use crate::algebra::structures::{CommutativeSemigroup, Monoid, Semigroup};
use crate::cmp::Bounded;

use std::cmp;

/// A monoid that returns the minimum value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Min<T>(pub T);

impl<T> Semigroup for Min<T>
where
  T: Clone + Ord,
{
  fn op(&self, rhs: &Self) -> Self {
    Min(cmp::min(&self.0, &rhs.0).clone())
  }
}

impl<T> CommutativeSemigroup for Min<T> where T: Clone + Ord {}

impl<T> Monoid for Min<T>
where
  T: Clone + Ord + Bounded,
{
  fn identity() -> Self {
    Min(T::MAX)
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
      iter().min().unwrap(),
      iter().fold(Min::identity(), |acc, x| acc.op(&Min(x))).0
    );
  }
}
