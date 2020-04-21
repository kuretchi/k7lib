use crate::algebra::structures::{AssociativeMagma, CommutativeMagma, Magma, UnitalMagma};
use crate::cmp::Bounded;

use std::cmp;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Max<T>(pub T);

impl<T> Magma for Max<T>
where
  T: Clone + Ord,
{
  fn op(&self, rhs: &Self) -> Self {
    Max(cmp::max(&self.0, &rhs.0).clone())
  }
}

impl<T> AssociativeMagma for Max<T> where T: Clone + Ord {}

impl<T> CommutativeMagma for Max<T> where T: Clone + Ord {}

impl<T> UnitalMagma for Max<T>
where
  T: Clone + Ord + Bounded,
{
  fn identity() -> Self {
    Max(T::min_value())
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
