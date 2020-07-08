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

  pub fn repr(&mut self, mut i: usize) -> usize {
    // Path halving
    while !self.is_root(i) {
      let j = self.nodes[self.nodes[i].parent].parent;
      //  ( i ) --> (   ) --> ( j )  or  ( i ) --> ( j ) -+
      //                                      ^-----------+
      self.nodes[i].parent = j;
      i = j;
      //  (   ) -+  (   ) --> ( i )  or  (   ) --> ( i ) -+
      //         +-----------^                ^-----------+
    }
    i
  }

  pub fn unite_sets(&mut self, i: usize, j: usize) -> bool {
    let mut i = self.repr(i);
    let mut j = self.repr(j);

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

    //  ( j ) -+  ( i ) -+
    // ^-------+ ^-------+
    self.nodes[j].parent = i;
    self.nodes[i].len += self.nodes[j].len;
    self.sets_len -= 1;
    //  ( j ) --> ( i ) -+
    //           ^-------+

    true
  }

  pub fn belong_to_same_set(&mut self, i: usize, j: usize) -> bool {
    self.repr(i) == self.repr(j)
  }

  pub fn set_len(&mut self, i: usize) -> usize {
    let i = self.repr(i);
    self.nodes[i].len
  }
}
