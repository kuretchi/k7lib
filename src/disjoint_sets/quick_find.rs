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
  /// Initially it consists of $n$ singletons:
  /// $\lbrace \lbrace 0 \rbrace, \lbrace 1 \rbrace, \dots, \lbrace n - 1 \rbrace \rbrace$,
  /// where $n$ = `len`.
  ///
  /// # Time complexity
  /// $\Theta(n)$
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
  /// $O(1)$
  pub fn len(&self) -> usize {
    self.reprs.len()
  }

  /// Returns the number of disjoint sets.
  ///
  /// # Time complexity
  /// $O(1)$
  pub fn sets_len(&self) -> usize {
    self.sets_len
  }

  /// Returns the representative of the set that the given element belongs to.
  ///
  /// # Time complexity
  /// $O(1)$
  pub fn find(&self, i: usize) -> usize {
    assert_index(i, self.len());

    self.reprs[i]
  }

  /// Unites two disjoint sets that the given elements belong to into one.
  ///
  /// Returns `false` iff two elements already belong to the same set.
  ///
  /// # Time complexity
  /// $O(\log(n))$ amortized
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
  /// $O(1)$
  pub fn belong_to_same_set(&self, i: usize, j: usize) -> bool {
    assert_index(i, self.len());
    assert_index(j, self.len());

    self.find(i) == self.find(j)
  }

  /// Returns the slice of elements that belong to the same set as the given element.
  ///
  /// # Time complexity
  /// $O(1)$
  pub fn set(&self, i: usize) -> &[usize] {
    assert_index(i, self.len());

    &self.elems[self.find(i)]
  }
}

#[cfg(test)]
mod tests {
  use super::{super::tests::NaiveDisjointSets, *};
  use quickcheck::{Arbitrary, Gen};
  use quickcheck_macros::quickcheck;
  use rand::Rng as _;
  use std::{cmp::Ordering::*, collections::HashSet};

  impl From<&QuickFind> for NaiveDisjointSets {
    fn from(uf: &QuickFind) -> Self {
      let mut sets = vec![None; uf.len()];
      for i in 0..uf.len() {
        sets[uf.find(i)].get_or_insert_with(HashSet::new).insert(i);
      }
      Self(sets)
    }
  }

  impl Arbitrary for QuickFind {
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
    let uf = QuickFind::new(len);

    assert_eq!(uf.len(), len);
    assert_eq!(uf.sets_len(), len);

    for i in 0..len {
      assert_eq!(uf.find(i), i);
    }

    for i in 0..len {
      for j in 0..len {
        assert_eq!(uf.belong_to_same_set(i, j), i == j);
      }
    }

    for i in 0..len {
      assert_eq!(uf.set(i), [i]);
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
  fn sets_len_prop(uf: QuickFind) {
    let ds = NaiveDisjointSets::from(&uf);
    assert_eq!(uf.sets_len(), ds.sets_len());
  }

  #[quickcheck]
  fn find_prop(uf: QuickFind, i: usize) {
    precondition!(uf, i);

    let ds = NaiveDisjointSets::from(&uf);
    assert_eq!(uf.find(i), ds.find(i));
  }

  #[quickcheck]
  fn unite_prop(mut uf: QuickFind, i: usize, j: usize) {
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
  fn set_prop(uf: QuickFind, i: usize) {
    precondition!(uf, i);

    let ds = NaiveDisjointSets::from(&uf);
    assert_eq!(uf.set(i).len(), ds.set_len(i));
    assert_eq!(
      &uf.set(i).iter().copied().collect::<HashSet<_>>(),
      ds.set(i)
    );
  }
}
