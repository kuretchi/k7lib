use crate::utils::index_bounds_check::assert_index;

use std::mem;

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

  pub fn new(len: usize) -> Self {
    Self {
      // Initially all nodes are root.
      nodes: (0..len).map(|i| Node { parent: i, len: 1 }).collect(),
      sets_len: len,
    }
  }

  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  pub fn sets_len(&self) -> usize {
    self.sets_len
  }

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

  pub fn belong_to_same_set(&mut self, i: usize, j: usize) -> bool {
    assert_index(i, self.len());
    assert_index(j, self.len());

    self.find(i) == self.find(j)
  }

  pub fn set_len(&mut self, i: usize) -> usize {
    assert_index(i, self.len());

    let i = self.find(i);
    self.nodes[i].len
  }
}
