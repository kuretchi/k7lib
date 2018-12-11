//! Data structures representing a sequence.

pub use self::cumulative_sum::CumulativeSum;
pub use self::fenwick_tree::FenwickTree;
pub use self::segment_tree::SegmentTree;

pub mod cumulative_sum;
pub mod fenwick_tree;
pub mod segment_tree;

use std::ops::{Range, RangeTo};

macro_rules! assert_index {
  ($cond:expr, $index:expr, $len:expr) => {
    assert!(
      $cond,
      "index out of bounds: the len is {:?} but the index is {:?}",
      $len, $index
    )
  };
}

fn assert_index(index: usize, len: usize) {
  assert_index!(index < len, index, len);
}

fn assert_index_range_to(index: &RangeTo<usize>, len: usize) {
  assert_index!(index.end <= len, index, len);
}

fn assert_index_range(index: &Range<usize>, len: usize) {
  assert!(
    index.start <= index.end,
    "range start is greater than range end: {:?}",
    index
  );
  assert_index!(index.end <= len, index, len);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn assert_index_test_1() {
    assert_index(100, 100);
  }

  #[test]
  #[should_panic]
  fn assert_index_test_2() {
    assert_index(0, 0);
  }

  #[test]
  fn assert_index_test_3() {
    assert_index(0, 1);
  }

  #[test]
  fn assert_index_test_4() {
    assert_index(99, 100);
  }

  #[test]
  #[should_panic]
  fn assert_index_range_to_test_1() {
    assert_index_range_to(&(..101), 100);
  }

  #[test]
  fn assert_index_range_to_test_2() {
    assert_index_range_to(&(..0), 0);
  }

  #[test]
  fn assert_index_range_to_test_3() {
    assert_index_range_to(&(..100), 100);
  }

  #[test]
  #[should_panic]
  fn assert_index_range_test_1() {
    assert_index_range(&(100..201), 200);
  }

  #[test]
  #[should_panic]
  fn assert_index_range_test_2() {
    assert_index_range(&(101..100), 200);
  }

  #[test]
  fn assert_index_range_test_3() {
    assert_index_range(&(0..0), 0);
  }

  #[test]
  fn assert_index_range_test_4() {
    assert_index_range(&(100..200), 200);
  }
}
