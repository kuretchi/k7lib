use crate::utils::index_bounds_check::assert_index;

use std::mem;

/// A disjoint-set data structure based on the weighted quick-union algorithm.
#[derive(Clone, Debug)]
pub struct QuickUnion {
  nodes: Vec<Node>,
  sets_len: usize,
}

#[derive(Clone, Debug)]
struct Node {
  parent: usize,
  len: usize,
}

impl QuickUnion {
  fn is_root(&self, i: usize) -> bool {
    self.nodes[i].parent == i
  }

  /// Creates a new `QuickUnion` with the given number of elements.
  ///
  /// Initially it consists of _n_ singletons: {{0}, {1}, ..., {_n_ - 1}},
  /// where _n_ = `len`.
  ///
  /// # Time complexity
  /// Θ(_n_)
  pub fn new(len: usize) -> Self {
    Self {
      // Initially all nodes are root.
      nodes: (0..len).map(|i| Node { parent: i, len: 1 }).collect(),
      sets_len: len,
    }
  }

  /// Returns the total number of elements that belong to disjoint sets.
  ///
  /// # Time complexity
  /// O(1)
  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  /// Returns the number of disjoint sets.
  ///
  /// # Time complexity
  /// O(1)
  pub fn sets_len(&self) -> usize {
    self.sets_len
  }

  /// Returns the representative of the set that the given element belongs to.
  ///
  /// # Time complexity
  /// O(α(_n_)) amortized
  pub fn find(&mut self, mut i: usize) -> usize {
    assert_index(i, self.len());

    // Path halving
    while !self.is_root(i) {
      let j = self.nodes[self.nodes[i].parent].parent;
      //  (j)
      //   |           +---+
      //  ( )     or  (j)--+
      //   |           |
      //  (i)         (i)
      self.nodes[i].parent = j;
      i = j;
      //  (i)--+
      //   |   |       +---+
      //  ( )  |  or  (i)--+
      //       |       |
      //  ( )--+      ( )
    }
    i
  }

  /// Unites two disjoint sets that the given elements belong to into one.
  ///
  /// Returns `false` iff two elements already belong to the same set.
  ///
  /// # Time complexity
  /// O(α(_n_)) amortized
  pub fn unite(&mut self, i: usize, j: usize) -> bool {
    assert_index(i, self.len());
    assert_index(j, self.len());

    let mut i = self.find(i);
    let mut j = self.find(j);

    if i == j {
      return false;
    }
    // Union by size
    if self.nodes[i].len < self.nodes[j].len {
      mem::swap(&mut i, &mut j);
    }

    debug_assert!(self.is_root(i));
    debug_assert!(self.is_root(j));
    debug_assert!(self.nodes[i].len >= self.nodes[j].len);

    self.nodes[j].parent = i;
    self.nodes[i].len += self.nodes[j].len;
    self.sets_len -= 1;
    true
  }

  /// Returns `true` iff the given elements belong to the same set.
  ///
  /// # Time complexity
  /// O(α(_n_)) amortized
  pub fn belong_to_same_set(&mut self, i: usize, j: usize) -> bool {
    assert_index(i, self.len());
    assert_index(j, self.len());

    self.find(i) == self.find(j)
  }

  /// Returns the number of elements that belong to the same set as the given element.
  ///
  /// # Time complexity
  /// O(α(_n_)) amortized
  pub fn set_len(&mut self, i: usize) -> usize {
    assert_index(i, self.len());

    let i = self.find(i);
    self.nodes[i].len
  }
}
