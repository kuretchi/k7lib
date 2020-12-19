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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::disjoint_sets::tests::NaiveDisjointSets;
  use quickcheck::{Arbitrary, Gen};
  use quickcheck_macros::quickcheck;
  use rand::Rng as _;
  use std::{cmp::Ordering::*, collections::HashSet};

  impl From<&QuickUnion> for NaiveDisjointSets {
    fn from(uf: &QuickUnion) -> Self {
      let mut sets = vec![None; uf.len()];
      for i in 0..uf.len() {
        sets[uf.find_without_compaction(i)]
          .get_or_insert_with(HashSet::new)
          .insert(i);
      }
      Self(sets)
    }
  }

  impl QuickUnion {
    fn find_without_compaction(&self, mut i: usize) -> usize {
      while !self.is_root(i) {
        i = self.nodes[i].parent;
      }
      i
    }
  }

  fn assert_unchanged<T, F>(uf: &mut QuickUnion, f: F) -> (NaiveDisjointSets, T)
  where
    F: FnOnce(&mut QuickUnion) -> T,
  {
    let old = NaiveDisjointSets::from(&*uf);
    let t = f(uf);
    let new = NaiveDisjointSets::from(&*uf);
    assert_eq!(old, new);
    (old, t)
  }

  impl Arbitrary for QuickUnion {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
      let len = g.size();
      let mut uf = Self::new(len);
      if len != 0 {
        for _ in 0..g.gen_range(0, len - 1) {
          let i = g.gen_range(0, len);
          let j = g.gen_range(0, len);
          uf.unite(i, j);
        }
      }
      uf
    }
  }

  #[quickcheck]
  fn initial_state_prop(len: usize) {
    let uf = QuickUnion::new(len);

    for i in 0..len {
      assert!(uf.is_root(i));
    }

    assert_eq!(uf.len(), len);
    assert_eq!(uf.sets_len(), len);

    for i in 0..len {
      let mut uf = uf.clone();
      assert_eq!(uf.find(i), i);
    }

    for i in 0..len {
      for j in 0..len {
        let mut uf = uf.clone();
        assert_eq!(uf.belong_to_same_set(i, j), i == j);
      }
    }

    for i in 0..len {
      let mut uf = uf.clone();
      assert_eq!(uf.set_len(i), 1);
    }
  }

  macro_rules! precondition {
    ($uf:expr, $($i:ident),*) => {
      if $uf.len() == 0 {
        return;
      }
      $(let $i = $i % $uf.len();)*
    };
  }

  #[quickcheck]
  fn sets_len_prop(uf: QuickUnion) {
    let ds = NaiveDisjointSets::from(&uf);
    assert_eq!(uf.sets_len(), ds.sets_len());
  }

  #[quickcheck]
  fn find_prop(mut uf: QuickUnion, i: usize) {
    precondition!(uf, i);

    let (ds, r) = assert_unchanged(&mut uf, |uf| uf.find(i));
    assert_eq!(r, ds.find(i));
  }

  #[quickcheck]
  fn unite_prop(mut uf: QuickUnion, i: usize, j: usize) {
    precondition!(uf, i, j);

    let old = NaiveDisjointSets::from(&uf);
    let united = uf.unite(i, j);
    let new = NaiveDisjointSets::from(&uf);

    let already_united = old.find(i) == old.find(j);
    assert_eq!(united, !already_united);

    if already_united {
      assert_eq!(new, old);
    } else {
      let mut united_to_j = old.clone();
      united_to_j.union(j, i);
      let mut united_to_i = old.clone();
      united_to_i.union(i, j);

      match old.set_len(i).cmp(&old.set_len(j)) {
        Less => assert_eq!(new, united_to_j),
        Equal => assert!(new == united_to_j || new == united_to_i),
        Greater => assert_eq!(new, united_to_i),
      }
    }
  }

  #[quickcheck]
  fn set_len_prop(mut uf: QuickUnion, i: usize) {
    precondition!(uf, i);

    let (ds, len) = assert_unchanged(&mut uf, |uf| uf.set_len(i));
    assert_eq!(len, ds.set_len(i));
  }
}
