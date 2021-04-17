//! Prefix sums.

use crate::algebra::structures::{Group, Monoid};
use crate::utils::index_bounds_check::*;

use std::iter::FromIterator;
use std::ops::{Range, RangeTo};

/// Prefix sums.
///
/// # Space complexity
/// $O(n \log(\sigma))$
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct PrefixSum<T> {
  vec: Vec<T>,
}

impl<M: Monoid> PrefixSum<M> {
  /// Creates an empty sequence.
  ///
  /// # Time complexity
  /// $O(1)$
  pub fn new() -> Self {
    Self { vec: vec![M::identity()] }
  }

  /// Appends an element to the back of the sequence.
  ///
  /// # Time complexity
  /// $O(1)$ amortized
  pub fn push(&mut self, value: &M) {
    let value = self.vec.last().unwrap().op(value);
    self.vec.push(value);
  }

  /// Returns the length of the sequence.
  ///
  /// # Time complexity
  /// $O(1)$
  pub fn len(&self) -> usize {
    self.vec.len() - 1
  }

  /// Folds elements in the given prefix range with a monoid's binary operation.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// $O(1)$
  pub fn prefix_sum(&self, index: RangeTo<usize>) -> &M {
    assert_index_range_to(index, self.len());

    &self.vec[index.end]
  }
}

impl<G: Group> PrefixSum<G> {
  /// Returns an element at the given index.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// $O(1)$
  pub fn point_get(&self, index: usize) -> G {
    assert_index(index, self.len());

    self.range_sum(index..index + 1)
  }

  /// Folds elements in the given range with a group's binary operation.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// $O(1)$
  pub fn range_sum(&self, index: Range<usize>) -> G {
    assert_index_range(&index, self.len());

    // [s, e) = [s, e - 1] = [0, s - 1] ^ -1 * [0, e - 1] = [0, s) ^ -1 * [0, e)
    let l = self.prefix_sum(..index.start).invert();
    let r = self.prefix_sum(..index.end);

    l.op(r)
  }
}

impl<M: Monoid> FromIterator<M> for PrefixSum<M> {
  /// Creates a new `PrefixSum` from an iterator.
  ///
  /// # Time complexity
  /// $O(n)$
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = M>,
  {
    let iter = iter.into_iter();
    let mut vec = Vec::with_capacity(iter.size_hint().0);
    vec.push(M::identity());

    for (i, value) in iter.enumerate() {
      let sum = vec[i].op(&value);
      vec.push(sum);
    }

    vec.shrink_to_fit();
    PrefixSum { vec }
  }
}
