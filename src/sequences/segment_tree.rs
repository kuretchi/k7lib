//! A segment tree.

use super::*;
use crate::algebra::structures::Monoid;

use std::collections::VecDeque;
use std::iter::{self, FromIterator};
use std::mem;
use std::ops::{Deref, DerefMut, Range};

/// A segment tree.
///
/// ```text
///  +----------------------------------------------------------------------------+
///  |                                    0001                                    |
///  +----------------------------------------------------------------------------+
///  +------------------------------------+  +------------------------------------+
///  |                0010                |  |                0011                |
///  +------------------------------------+  +------------------------------------+
///  +----------------+  +----------------+  +----------------+  +----------------+
///  |      0100      |  |      0101      |  |      0110      |  |      0111      |
///  +----------------+  +----------------+  +----------------+  +----------------+
///  +------+  +------+  +------+  +------+  +------+  +------+  +------+  +------+
///  | 1000 |  | 1001 |  | 1010 |  | 1011 |  | 1100 |  | 1101 |  | 1110 |  | 1111 |
///  +------+  +------+  +------+  +------+  +------+  +------+  +------+  +------+
/// ```
///
/// # Space complexity
/// O(n log σ)
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SegmentTree<T> {
  vec: Vec<T>,

  // virtual length, which is power of two
  base_len: usize,
  // avaliable length
  len: usize,
}

impl<M: Monoid> FromIterator<M> for SegmentTree<M> {
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = M>,
  {
    let iter = iter.into_iter();

    let min_len = iter.size_hint().0;
    let (min_base_len, min_vec_len) = Self::extend_len(min_len);

    let mut deque = VecDeque::with_capacity(min_vec_len);

    if min_base_len > 1 {
      // inner nodes
      deque.extend(iter::repeat(M::identity()).take(min_base_len - 1));
    }

    // leaf nodes
    deque.extend(iter);

    let len = deque.len() - min_base_len.saturating_sub(1);
    let (base_len, _) = Self::extend_len(len);

    if base_len > min_base_len {
      for identity in iter::repeat(M::identity()).take(base_len - min_base_len) {
        deque.push_front(identity);
      }
    // for buggy iterator
    } else if min_base_len > base_len {
      deque.drain(..min_base_len - base_len);
    }

    let mut tree = SegmentTree {
      vec: deque.into(),
      base_len: base_len,
      len: len,
    };

    for node in (1..base_len).rev() {
      tree.recalc(node);
    }

    tree
  }
}

impl<M: Monoid> SegmentTree<M> {
  /// Creates a new `SegmentTree` of the given length, filled with an identity element.
  ///
  /// # Panics
  /// Panics if `len.next_power_of_two() - 1 + len` overflows `usize`.
  ///
  /// # Time complexity
  /// O(n)
  pub fn new(len: usize) -> Self {
    let (base_len, vec_len) = Self::extend_len(len);

    let vec = if vec_len == 0 {
      vec![]
    } else {
      vec![M::identity(); vec_len]
    };

    SegmentTree {
      vec: vec,
      base_len: base_len,
      len: len,
    }
  }

  /// Returns the length of the sequence.
  ///
  /// # Time complexity
  /// O(1)
  pub fn len(&self) -> usize {
    self.len
  }

  /// Returns an element at the given index.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// O(1)
  pub fn get(&self, index: usize) -> &M {
    assert_index(index, self.len());

    self.node(self.node_index(index))
  }

  /// Returns a mutable reference to an element at the given index.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// O(1) (`GetMut::drop`: O(log n))
  pub fn get_mut(&mut self, index: usize) -> GetMut<M> {
    assert_index(index, self.len());

    GetMut {
      node: self.node_index(index),
      tree: self,
    }
  }

  /// Folds elements in the given range with a monoid's binary operation.
  ///
  /// # Panics
  /// Panics if `index` is out of bounds.
  ///
  /// # Time complexity
  /// O(log n)
  pub fn fold(&self, index: Range<usize>) -> M {
    assert_index_range(&index, self.len());

    let mut start = self.node_index(index.start);
    let mut end = self.node_index(index.end);

    let mut lacc = M::identity();
    let mut racc = M::identity();

    while start < end {
      if start & 1 == 1 {
        lacc.op_assign_right(self.node(start));

        // [     010     ] [     011     ]
        // [ 100 ] [ 101 ] [ 110 ] [ 111 ]
        //            *  -->  *
        start += 1;
      }

      if end & 1 == 1 {
        // [     010     ] [     011     ]
        // [ 100 ] [ 101 ] [ 110 ] [ 111 ]
        //                    *  <--  *
        end -= 1;

        racc.op_assign_left(self.node(end));
      }

      // move to parents
      start >>= 1;
      end >>= 1;
    }

    lacc.op(&racc)
  }

  // (base_len, vec_len)
  fn extend_len(len: usize) -> (usize, usize) {
    if len == 0 {
      (0, 0)
    } else {
      len
        .checked_next_power_of_two()
        .and_then(|base_len| {
          (base_len - 1)
            .checked_add(len)
            .map(|vec_len| (base_len, vec_len))
        })
        .unwrap_or_else(|| panic!("length too large: {:?}", len))
    }
  }

  fn node_index(&self, index: usize) -> usize {
    self.base_len + index
  }

  fn recalc(&mut self, node: usize) {
    let l = node << 1;
    let r = (node << 1) | 1;

    // inclusive
    let last = self.vec.len();
    debug_assert_eq!(last, self.node_index(self.len() - 1));

    if l <= last {
      *self.node_mut(node) = if r <= last {
        self.node(l).op(&self.node(r))
      } else {
        self.node(l).clone()
      };
    }
  }

  fn rebuild(&mut self, mut node: usize) {
    while {
      node >>= 1;
      node > 0
    } {
      self.recalc(node);
    }
  }

  fn node(&self, node: usize) -> &M {
    &self.vec[node - 1]
  }

  fn node_mut(&mut self, node: usize) -> &mut M {
    &mut self.vec[node - 1]
  }
}

/// Structure wrapping a mutable refenrece to an element on `SegmentTree`.
pub struct GetMut<'a, M: 'a + Monoid> {
  tree: &'a mut SegmentTree<M>,
  node: usize,
}

impl<'a, M: Monoid> Drop for GetMut<'a, M> {
  fn drop(&mut self) {
    self.tree.rebuild(self.node);
  }
}

impl<'a, M: Monoid> Deref for GetMut<'a, M> {
  type Target = M;

  fn deref(&self) -> &M {
    self.tree.node(self.node)
  }
}

impl<'a, M: Monoid> DerefMut for GetMut<'a, M> {
  fn deref_mut(&mut self) -> &mut M {
    self.tree.node_mut(self.node)
  }
}

impl<'a, M: Monoid> GetMut<'a, M> {
  /// Updates the value using the given function.
  pub fn update<F>(&mut self, f: F)
  where
    F: FnOnce(M) -> M,
  {
    let value = mem::replace::<M>(self, M::identity());
    mem::replace::<M>(self, f(value));
  }
}
