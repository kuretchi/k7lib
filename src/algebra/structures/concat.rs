use crate::algebra::structures::{Monoid, Semigroup};

/// A monoid under string concatenation.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Concat<T>(pub Vec<T>);

impl<T> Semigroup for Concat<T>
where
  T: Clone,
{
  fn op(&self, rhs: &Self) -> Self {
    Self(self.0.iter().cloned().chain(rhs.0.iter().cloned()).collect())
  }
}

impl<T> Monoid for Concat<T>
where
  T: Clone,
{
  fn identity() -> Self {
    Self(vec![])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Concat(vec![0, 1, 2]).op(&Concat::identity()).op(&Concat(vec![3, 4])),
      Concat(vec![0, 1, 2, 3, 4])
    );
  }
}
