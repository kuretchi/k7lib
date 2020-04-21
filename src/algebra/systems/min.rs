use crate::algebra::structures::{AssociativeMagma, CommutativeMagma, Magma, UnitalMagma};
use crate::cmp::Bounded;

use std::cmp;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Min<T>(pub T);

impl<T> Magma for Min<T>
where
  T: Clone + Ord,
{
  fn op(&self, rhs: &Self) -> Self {
    Min(cmp::min(&self.0, &rhs.0).clone())
  }
}

impl<T> AssociativeMagma for Min<T> where T: Clone + Ord {}

impl<T> CommutativeMagma for Min<T> where T: Clone + Ord {}

impl<T> UnitalMagma for Min<T>
where
  T: Clone + Ord + Bounded,
{
  fn identity() -> Self {
    Min(T::max_value())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    let iter = || vec.iter().cloned();

    assert_eq!(
      iter().min().unwrap(),
      iter().fold(Min::identity(), |acc, x| acc.op(&Min(x))).0
    );
  }
}
