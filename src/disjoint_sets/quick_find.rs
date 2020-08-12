use crate::utils::index_bounds_check::*;

use std::mem;

#[derive(Clone, Debug)]
pub struct QuickFind {
  elems: Vec<Vec<usize>>,
  reprs: Vec<usize>,
  sets_len: usize,
}

impl QuickFind {
  pub fn new(len: usize) -> Self {
    Self {
      elems: (0..len).map(|i| vec![i]).collect(),
      reprs: (0..len).collect(),
      sets_len: len,
    }
  }

  pub fn len(&self) -> usize {
    self.reprs.len()
  }

  pub fn sets_len(&self) -> usize {
    self.sets_len
  }

  pub fn repr(&self, i: usize) -> usize {
    assert_index(i, self.len());

    self.reprs[i]
  }

  pub fn unite_sets(&mut self, i: usize, j: usize) -> bool {
    assert_index(i, self.len());
    assert_index(j, self.len());

    let mut i = self.repr(i);
    let mut j = self.repr(j);

    if i == j {
      return false;
    }
    // Union by size
    if self.elems[i].len() < self.elems[j].len() {
      mem::swap(&mut i, &mut j);
    }

    debug_assert!(self.elems[i].len() >= self.elems[j].len());

    for &k in &self.elems[j] {
      self.reprs[k] = i;
    }
    let mut elems_j = mem::replace(&mut self.elems[j], vec![]);
    self.elems[i].append(&mut elems_j);
    self.sets_len -= 1;
    true
  }

  pub fn belong_to_same_set(&self, i: usize, j: usize) -> bool {
    assert_index(i, self.len());
    assert_index(j, self.len());

    self.repr(i) == self.repr(j)
  }

  pub fn set_len(&self, i: usize) -> usize {
    assert_index(i, self.len());

    self.elems[self.repr(i)].len()
  }
}
