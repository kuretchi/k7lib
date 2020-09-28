use crate::algebra::structures::{AssociativeMagma, Magma};

/// A left zero semigroup.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct First<T>(pub T);

impl<T> Magma for First<T>
where
  T: Clone,
{
  fn op(&self, _: &Self) -> Self {
    First(self.0.clone())
  }
}

impl<T> AssociativeMagma for First<T> where T: Clone {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(First(3).op(&First(1)).op(&First(4)), First(3));
  }
}
