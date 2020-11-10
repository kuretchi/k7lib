use std::cmp::Ordering::{self, *};

/// Permutes the slice into the next greater permutation in lexicographically order.
///
/// If successful, `true` is returned.
/// If the next permutation does not exist, nothing is performed and `false` is returned.
///
/// # Examples
///
/// ```
/// # use spella::slice::next_permutation;
/// let mut s = [0, 0, 2, 1];
/// assert!(next_permutation(&mut s));
/// assert_eq!(s, [0, 1, 0, 2]);
/// ```
///
/// ```
/// # use spella::slice::next_permutation;
/// let mut s = [2, 1, 0, 0];
/// assert!(!next_permutation(&mut s));
/// assert_eq!(s, [2, 1, 0, 0]);
/// ```
///
/// Enumerating all permutations:
/// ```
/// # use spella::slice::next_permutation;
/// let mut s = [0, 0, 1, 2]; // sorted
/// loop {
///   println!("{:?}", s);
///   if !next_permutation(&mut s) {
///     break;
///   }
/// }
/// ```
///
/// # Time complexity
///
/// * Θ(n) worst per operation
/// * Θ(n!) worst, to enumerate all permutations
pub fn next_permutation<T>(s: &mut [T]) -> bool
where
  T: Ord,
{
  next_permutation_by(s, T::cmp)
}

/// Permutes the slice into the next greater permutation in lexicographically order
/// with a comparator function.
///
/// See [`next_permutation`](./fn.next_permutation.html) for details.
///
/// # Examples
///
/// ```
/// # use spella::slice::next_permutation_by;
/// let mut s = [2, 1, 0];
/// assert!(next_permutation_by(&mut s, |l, r| l.cmp(r).reverse()));
/// assert_eq!(s, [2, 0, 1]);
/// ```
pub fn next_permutation_by<T, F>(s: &mut [T], mut cmp: F) -> bool
where
  F: FnMut(&T, &T) -> Ordering,
{
  let i = match s.windows(2).rposition(|w| cmp(&w[0], &w[1]) == Less) {
    Some(i) => i,
    None => return false,
  };
  let j = s.iter().rposition(|c| cmp(c, &s[i]) == Greater).unwrap();
  //     i        j
  // [3, 2, 5, 4, 4, 2, 1]
  //        ^^^^^^^^^^^^^ sorted in descending order
  s.swap(i, j);
  //     i
  // [3, 4, 5, 4, 2, 2, 1]
  //        ^^^^^^^^^^^^^ sorted in descending order
  s[i + 1..].reverse();
  // [3, 4, 1, 2, 2, 4, 5]
  //        ^^^^^^^^^^^^^ sorted in ascending order
  true
}

/// Permutes the slice into the next greater permutation in lexicographically order
/// with a key extraction function.
///
/// See [`next_permutation`](./fn.next_permutation.html) for details.
///
/// # Examples
///
/// ```
/// # use spella::slice::next_permutation_by_key;
/// use std::cmp::Reverse;
///
/// let mut s = [2, 1, 0];
/// assert!(next_permutation_by_key(&mut s, |&c| Reverse(c)));
/// assert_eq!(s, [2, 0, 1]);
/// ```
pub fn next_permutation_by_key<T, K, F>(s: &mut [T], mut f: F) -> bool
where
  F: FnMut(&T) -> K,
  K: Ord,
{
  next_permutation_by(s, |l, r| f(l).cmp(&f(r)))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fmt::Debug;

  fn assert_next<T>(s: &mut [T], t: &[T])
  where
    T: Ord + Debug,
  {
    assert!(next_permutation(s));
    assert_eq!(s, t);
  }

  fn assert_last<T>(s: &mut [T])
  where
    T: Clone + Ord + Debug,
  {
    let t = s.to_vec();
    assert!(!next_permutation(s));
    assert_eq!(s, t.as_slice());
  }

  #[test]
  fn distinct() {
    let s = &mut [0, 1, 2, 3];

    assert_next(s, &[0, 1, 3, 2]);
    assert_next(s, &[0, 2, 1, 3]);
    assert_next(s, &[0, 2, 3, 1]);
    assert_next(s, &[0, 3, 1, 2]);
    assert_next(s, &[0, 3, 2, 1]);
    assert_next(s, &[1, 0, 2, 3]);
    assert_next(s, &[1, 0, 3, 2]);
    assert_next(s, &[1, 2, 0, 3]);
    assert_next(s, &[1, 2, 3, 0]);
    assert_next(s, &[1, 3, 0, 2]);
    assert_next(s, &[1, 3, 2, 0]);
    assert_next(s, &[2, 0, 1, 3]);
    assert_next(s, &[2, 0, 3, 1]);
    assert_next(s, &[2, 1, 0, 3]);
    assert_next(s, &[2, 1, 3, 0]);
    assert_next(s, &[2, 3, 0, 1]);
    assert_next(s, &[2, 3, 1, 0]);
    assert_next(s, &[3, 0, 1, 2]);
    assert_next(s, &[3, 0, 2, 1]);
    assert_next(s, &[3, 1, 0, 2]);
    assert_next(s, &[3, 1, 2, 0]);
    assert_next(s, &[3, 2, 0, 1]);
    assert_next(s, &[3, 2, 1, 0]);

    assert_last(s);
  }

  #[test]
  fn not_distinct() {
    let s = &mut [0, 1, 1, 2];

    assert_next(s, &[0, 1, 2, 1]);
    assert_next(s, &[0, 2, 1, 1]);
    assert_next(s, &[1, 0, 1, 2]);
    assert_next(s, &[1, 0, 2, 1]);
    assert_next(s, &[1, 1, 0, 2]);
    assert_next(s, &[1, 1, 2, 0]);
    assert_next(s, &[1, 2, 0, 1]);
    assert_next(s, &[1, 2, 1, 0]);
    assert_next(s, &[2, 0, 1, 1]);
    assert_next(s, &[2, 1, 0, 1]);
    assert_next(s, &[2, 1, 1, 0]);

    assert_last(s);
  }

  #[test]
  fn all_equal() {
    let s = &mut [0, 0, 0, 0];
    assert_last(s);
  }

  #[test]
  fn single() {
    let s = &mut [0];
    assert_last(s);
  }

  #[test]
  fn empty() {
    let s: &mut [i32] = &mut [];
    assert_last(s);
  }
}
