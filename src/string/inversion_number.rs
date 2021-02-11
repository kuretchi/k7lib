use crate::num::primitive::Int as PrimInt;
use std::{convert::TryFrom, iter, mem};

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
  let len = if s.len() <= 1 { 0 } else { s.len() };
  let mut t = iter::repeat_with(T::default).take(len).collect::<Vec<_>>();
  go(s, &mut t)
}

// Calculates the inversion number of `s`, performing merge sort.
// Uses `t` as an auxiliary space.
fn go<T, Int>(s: &mut [T], t: &mut [T]) -> Int
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
  let t = &mut t[..s.len()];
  let mut l = 0;
  let mut r = m;
  for t_i in t.iter_mut() {
    if l < m && (r == s.len() || s[l] <= s[r]) {
      mem::swap(t_i, &mut s[l]);
      l += 1;
    } else {
      // `s[r]` jumps over `s[l..m]`
      cnt += Int::try_from(m - l).ok().unwrap();
      mem::swap(t_i, &mut s[r]);
      r += 1;
    }
  }
  s.swap_with_slice(t);
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
