use std::iter::FusedIterator;

/// Creates a run-length encoded iterator of the given iterator.
///
/// # Examples
/// ```
/// # use spella::string::run_length_encode;
/// let mut iter = run_length_encode(b"aaabcccc");
/// assert_eq!(iter.next(), Some((&b'a', 3)));
/// assert_eq!(iter.next(), Some((&b'b', 1)));
/// assert_eq!(iter.next(), Some((&b'c', 4)));
/// assert_eq!(iter.next(), None);
/// ```
pub fn run_length_encode<I>(iter: I) -> RunLengthEncode<I::IntoIter>
where
  I: IntoIterator,
  I::Item: PartialEq,
{
  RunLengthEncode {
    iter: iter.into_iter(),
    current: None,
  }
}

/// An iterator created by [`run_length_encode`].
#[derive(Clone, Debug)]
pub struct RunLengthEncode<I>
where
  I: Iterator,
{
  iter: I,
  current: Option<I::Item>,
}

impl<I> Iterator for RunLengthEncode<I>
where
  I: Iterator,
  I::Item: PartialEq,
{
  type Item = (I::Item, usize);

  fn next(&mut self) -> Option<Self::Item> {
    let current = self.current.take().or_else(|| self.iter.next())?;
    let mut cnt = 1;
    while let Some(next) = self.iter.next() {
      if next != current {
        self.current = Some(next);
        break;
      }
      cnt += 1;
    }
    Some((current, cnt))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let (lower, upper) = self.iter.size_hint();
    let lower = usize::from(self.current.is_some() || lower != 0);
    let upper = upper.and_then(|u| u.checked_add(self.current.as_ref().map_or(0, |_| 1)));
    (lower, upper)
  }
}

impl<I> FusedIterator for RunLengthEncode<I>
where
  I: FusedIterator,
  I::Item: PartialEq,
{
}

#[cfg(test)]
mod tests {
  use super::*;

  use quickcheck_macros::quickcheck;
  use std::convert::identity;
  use std::iter;
  use std::usize;

  #[test]
  fn next_test() {
    let mut vec = vec![
      Some(3),
      Some(3),
      None,
      Some(3),
      None,
      None,
      Some(3),
      Some(3),
      Some(2),
      Some(1),
      Some(0),
      Some(0),
      Some(0),
    ];
    let mut iter = run_length_encode(iter::from_fn(|| vec.pop().and_then(identity)));

    assert_eq!(iter.next(), Some((0, 3)));
    assert_eq!(iter.next(), Some((1, 1)));
    assert_eq!(iter.next(), Some((2, 1)));
    assert_eq!(iter.next(), Some((3, 2)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), Some((3, 1)));
    assert_eq!(iter.next(), Some((3, 2)));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn size_hint_test_1() {
    let vec = vec![0, 0, 0, 1, 2, 3, 3];
    let mut iter = run_length_encode(vec);

    // [0, 0, 0, 1, 2, 3, 3]
    assert_eq!(iter.size_hint(), (1, Some(7)));
    iter.next();
    // [1, 2, 3, 3]
    assert_eq!(iter.size_hint(), (1, Some(4)));
    iter.next();
    // [2, 3, 3]
    assert_eq!(iter.size_hint(), (1, Some(3)));
    iter.next();
    // [3, 3]
    assert_eq!(iter.size_hint(), (1, Some(2)));
    iter.next();
    // []
    assert_eq!(iter.size_hint(), (0, Some(0)));
  }

  #[test]
  fn size_hint_test_2() {
    struct Iter(usize);

    impl Iterator for Iter {
      type Item = usize;

      fn next(&mut self) -> Option<usize> {
        if self.0 != 0 {
          self.0 -= 1;
          Some(self.0)
        } else {
          None
        }
      }

      fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(usize::MAX))
      }
    }

    let mut iter = run_length_encode(Iter(10));

    assert_eq!(iter.size_hint(), (0, Some(usize::MAX)));
    iter.next();
    assert_eq!(iter.size_hint(), (1, None));
  }

  #[quickcheck]
  fn prop_1(vec: Vec<u8>) {
    let vec2 = run_length_encode(vec.iter().copied())
      .flat_map(|(x, n)| iter::repeat(x).take(n))
      .collect::<Vec<_>>();

    assert_eq!(vec, vec2);
  }

  #[quickcheck]
  fn prop_2(mut rle: Vec<(u8, usize)>) {
    rle.retain(|&(_, n)| n != 0);
    rle.dedup_by_key(|&mut (x, _)| x);

    let vec = rle
      .iter()
      .copied()
      .flat_map(|(x, n)| iter::repeat(x).take(n))
      .collect::<Vec<_>>();
    let rle2 = run_length_encode(vec).collect::<Vec<_>>();

    assert_eq!(rle, rle2);
  }
}
