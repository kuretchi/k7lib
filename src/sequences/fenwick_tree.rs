//! A fenwick tree (a.k.a. binary indexed tree).

use super::*;
use crate::algebra::structures::{AbelianGroup, CommutativeMagma, Monoid};

use std::iter::FromIterator;
use std::ops::{Range, RangeTo};

/// A fenwick tree (a.k.a. binary indexed tree).
///
/// ```text
///  +-----------------------------------------------------------------------------+
///  |                                     1000                                    |
///  +-----------------------------------------------------------------------------+
/// 0  ^                                       ^                   ^                8
///    | +100                                  |                   |
///  +-------------------------------------+   |                   |
///  |                0 100                |   |                   |
///  +-------------------------------------+   |                   |
/// 0  ^                   ^         ^      4  |                   |
///    | +10               |         |         | +10               |
///  +-----------------+   |         | -10   +-----------------+   |
///  |      00 10      |   |         +------ |      01 10      |   |
///  +-----------------+   |         |       +-----------------+   |
/// 0  ^         ^      2  |         |      4  ^         ^      6  |
///    | +1      |         | +1      |         | +1      |         | +1
///  +-------+   | -1    +-------+   | -1    +-------+   | -1    +-------+
///  | 000 1 |   +------ | 001 1 |   +------ | 010 1 |   +------ | 011 1 |
///  +-------+           +-------+           +-------+           +-------+
/// 0         1         2         3         4         5         6         7
/// ```
///
/// # Space complexity
/// O(n log σ)
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FenwickTree<T> {
  vec: Vec<T>,
}

impl<M: Monoid + CommutativeMagma> FenwickTree<M> {
  /// Creates a new `FenwickTree` of the given length, filled with an identity element.
  ///
  /// # Time complexity
  /// O(n)
  pub fn new(len: usize) -> Self {
    FenwickTree {
      vec: vec![M::identity(); len],
    }
  }

  /// Returns the length of the sequence.
  ///
  /// # Time complexity
  /// O(1)
  pub fn len(&self) -> usize {
    self.vec.len()
  }

  /// Append the given value to an element at the given index with a monoid's binary operation.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// O(log n)
  pub fn point_append(&mut self, index: usize, value: &M) {
    assert_index(index, self.len());

    // 0-based => 1-based
    let mut node = index + 1;

    while node <= self.len() {
      self.node_mut(node).op_assign_right(value);
      node += lsb(node);
    }
  }

  /// Folds elements in the given prefix range with a monoid's binary operation.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// O(log n)
  pub fn prefix_fold(&self, index: RangeTo<usize>) -> M {
    assert_index_range_to(index, self.len());

    // 0-based [0, e) => 1-based [1, e + 1) => 1-based [1, e]
    let mut end = index.end;

    let mut acc = M::identity();

    while end > 0 {
      acc.op_assign_right(self.node(end));
      end -= lsb(end);
    }

    acc
  }

  fn node(&self, node: usize) -> &M {
    &self.vec[node - 1]
  }

  fn node_mut(&mut self, node: usize) -> &mut M {
    &mut self.vec[node - 1]
  }
}

impl<G: AbelianGroup> FenwickTree<G> {
  /// Returns an element at the given index.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// O(log n)
  pub fn get(&self, index: usize) -> G {
    assert_index(index, self.len());

    self.fold(index..index + 1)
  }

  /// Replaces an element at the given index with the given value, and returns the old one.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// O(log n)
  pub fn replace(&mut self, index: usize, value: &G) -> G {
    let old_value = self.get(index);
    self.point_append(index, &value.inverse_op(&old_value));
    old_value
  }

  /// Folds elements in the given range with a group's binary operation.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// O(log n)
  pub fn fold(&self, index: Range<usize>) -> G {
    assert_index_range(&index, self.len());

    // 0-based [s, e) => 1-based [s + 1, e + 1) => 1-based (s, e]
    let mut start = index.start;
    let mut end = index.end;

    let mut acc = G::identity();

    while start < end {
      acc.op_assign_right(self.node(end));
      end -= lsb(end);
    }

    while end < start {
      acc.inverse_op_assign_right(self.node(start));
      start -= lsb(start);
    }

    acc
  }
}

impl<M: Monoid + CommutativeMagma> From<Vec<M>> for FenwickTree<M> {
  /// Creates a new `FenwickTree` from a `Vec`.
  ///
  /// # Time complexity
  /// O(n)
  fn from(mut vec: Vec<M>) -> Self {
    vec.shrink_to_fit();
    let mut tree = FenwickTree { vec };

    // the last element (`tree.node(tree.len())`) does not have a parent
    for node in 1..tree.len() {
      let parent = node + lsb(node);

      if parent <= tree.len() {
        *tree.node_mut(parent) = tree.node(parent).op(&tree.node(node));
      }
    }

    tree
  }
}

impl<M: Monoid + CommutativeMagma> FromIterator<M> for FenwickTree<M> {
  /// Creates a new `FenwickTree` from an iterator.
  ///
  /// # Time complexity
  /// O(n)
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = M>,
  {
    Self::from(iter.into_iter().collect::<Vec<_>>())
  }
}

// least significant bit
fn lsb(x: usize) -> usize {
  debug_assert_ne!(x, 0);
  x & (!x + 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lsb_test() {
    assert_eq!(lsb(0b1_10000), 0b10000);
    assert_eq!(lsb(0b00_1000), 0b_1000);
    assert_eq!(lsb(0b010_100), 0b__100);
    assert_eq!(lsb(0b1101_10), 0b___10);
    assert_eq!(lsb(0b11111_1), 0b____1);
    assert_eq!(lsb(usize::max_value()), 1);
  }
}
