use std::cmp::Ordering::{self, *};

pub fn next_permutation<T>(s: &mut [T]) -> bool
where
  T: Ord,
{
  next_permutation_by(s, T::cmp)
}

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

pub fn next_permutation_by_key<T, K, F>(s: &mut [T], mut f: F) -> bool
where
  F: FnMut(&T) -> K,
  K: Ord,
{
  next_permutation_by(s, |l, r| f(l).cmp(&f(r)))
}
