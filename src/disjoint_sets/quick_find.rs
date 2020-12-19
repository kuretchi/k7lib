use crate::utils::index_bounds_check::assert_index;

use std::mem;

/// A disjoint-set data structure based on the weighted quick-find algorithm.
///
/// # References
///
/// * ["データ構造をマージする一般的なテク" とは？ - (iwi) ｛ 反省します - TopCoder部][1]
///
/// [1]: https://web.archive.org/web/20181213115442/http://topcoder.g.hatena.ne.jp/iwiwi/20131226/1388062106
#[derive(Clone, Debug)]
pub struct QuickFind {
  elems: Vec<Vec<usize>>,
  reprs: Vec<usize>,
  sets_len: usize,
}

impl QuickFind {
  /// Creates a new `QuickFind` with the given number of elements.
  ///
  /// Initially it consists of _n_ singletons: {{0}, {1}, ..., {_n_ - 1}},
  /// where _n_ = `len`.
  ///
  /// # Time complexity
  /// Θ(_n_)
  pub fn new(len: usize) -> Self {
    Self {
      elems: (0..len).map(|i| vec![i]).collect(),
      reprs: (0..len).collect(),
      sets_len: len,
    }
  }

  /// Returns the total number of elements that belong to disjoint sets.
  ///
  /// # Time complexity
  /// O(1)
  pub fn len(&self) -> usize {
    self.reprs.len()
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
  /// O(1)
  pub fn find(&self, i: usize) -> usize {
    assert_index(i, self.len());

    self.reprs[i]
  }

  /// Unites two disjoint sets that the given elements belong to into one.
  ///
  /// Returns `false` iff two elements already belong to the same set.
  ///
  /// # Time complexity
  /// O(log(_n_)) amortized
  pub fn unite(&mut self, i: usize, j: usize) -> bool {
    assert_index(i, self.len());
    assert_index(j, self.len());

    let mut i = self.find(i);
    let mut j = self.find(j);

    if i == j {
      return false;
    }
    // Union by size
    if self.elems[i].len() < self.elems[j].len() {
      mem::swap(&mut i, &mut j);
    }

    debug_assert!(self.elems[i].len() >= self.elems[j].len());

    for &elem_in_j in &self.elems[j] {
      self.reprs[elem_in_j] = i;
    }
    // TODO: Use `mem::take` since 1.40.0
    let mut elems_in_j = mem::replace(&mut self.elems[j], vec![]);
    self.elems[i].append(&mut elems_in_j);
    self.sets_len -= 1;
    true
  }

  /// Returns `true` iff the given elements belong to the same set.
  ///
  /// # Time complexity
  /// O(1)
  pub fn belong_to_same_set(&self, i: usize, j: usize) -> bool {
    assert_index(i, self.len());
    assert_index(j, self.len());

    self.find(i) == self.find(j)
  }

  /// Returns the slice of elements that belong to the same set as the given element.
  ///
  /// # Time complexity
  /// O(1)
  pub fn set(&self, i: usize) -> &[usize] {
    assert_index(i, self.len());

    &self.elems[self.find(i)]
  }
}
