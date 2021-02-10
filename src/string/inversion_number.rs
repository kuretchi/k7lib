use crate::num::primitive::Int as PrimInt;
use std::{convert::TryFrom, mem};

/// Calculates the inversion number.
///
/// This function uses its argument destructively.
///
/// # Examples
/// ```
/// # use k7lib::string::inversion_number;
/// let mut s = [3, 1, 4, 1, 5];
/// assert_eq!(inversion_number::<_, u32>(&mut s), 3);
/// ```
///
/// # Time complexity
/// $\Theta(n \log(n))$ where $n$ = `s.len()`
pub fn inversion_number<T, Int>(s: &mut [T]) -> Int
where
  T: Ord + Default,
  Int: PrimInt + TryFrom<usize>,
{
  let cap = if s.len() <= 1 { 0 } else { s.len() };
  let mut t = Vec::with_capacity(cap);
  go(s, &mut t)
}

// Calculates the inversion number of `s`, performing merge sort.
// Uses `t` as an auxiliary space.
fn go<T, Int>(s: &mut [T], t: &mut Vec<T>) -> Int
where
  T: Ord + Default,
  Int: PrimInt + TryFrom<usize>,
{
  if s.len() <= 1 {
    return Int::ZERO;
  }
  let mut cnt = Int::ZERO;
  let m = s.len() / 2;
  cnt += go::<_, Int>(&mut s[..m], t);
  cnt += go::<_, Int>(&mut s[m..], t);
  debug_assert!(t.is_empty());
  let mut l = 0;
  let mut r = m;
  loop {
    if l < m && (r == s.len() || s[l] <= s[r]) {
      // TODO: use `mem::take` since 1.40.0
      t.push(mem::replace(&mut s[l], T::default()));
      l += 1;
    } else if r < s.len() {
      // `s[r]` jumps over `s[l..m]`
      cnt += Int::try_from(m - l).ok().unwrap();
      t.push(mem::replace(&mut s[r], T::default()));
      r += 1;
    } else {
      break;
    }
  }
  s.swap_with_slice(t);
  t.clear();
  cnt
}

#[cfg(test)]
mod tests {
  use super::*;
  use quickcheck_macros::quickcheck;

  fn naive<T>(s: &[T]) -> u64
  where
    T: Ord,
  {
    let mut cnt = 0;
    for i in 0..s.len() {
      for j in i + 1..s.len() {
        if s[i] > s[j] {
          cnt += 1;
        }
      }
    }
    cnt
  }

  fn test<T>(s: &mut [T])
  where
    T: Ord + Default,
  {
    let cnt = naive(s);
    assert_eq!(inversion_number::<_, u64>(s), cnt);
  }

  #[test]
  fn empty() {
    test::<i32>(&mut []);
  }

  #[test]
  fn single() {
    test(&mut [0]);
  }

  #[test]
  fn not_distinct() {
    test(&mut [2, 2, 3, 3, 2, 1, 1, 0]);
  }

  #[quickcheck]
  fn prop(mut v: Vec<i32>) {
    test(&mut v);
  }
}
