use std::cmp::Reverse;

/// Trait for types that have min/max values.
pub trait Bounded: Ord {
  const MIN: Self;
  const MAX: Self;
}

macro_rules! prim {
  ($($T:ident)*) => {$(
    impl Bounded for $T {
      const MIN: Self = std::$T::MIN;
      const MAX: Self = std::$T::MAX;
    }
  )*};
}

prim! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

impl Bounded for bool {
  const MIN: Self = false;
  const MAX: Self = true;
}

impl Bounded for () {
  const MIN: Self = ();
  const MAX: Self = ();
}

impl<T> Bounded for Option<T>
where
  T: Bounded,
{
  const MIN: Self = None;
  const MAX: Self = Some(T::MAX);
}

impl<T> Bounded for Reverse<T>
where
  T: Bounded,
{
  const MIN: Self = Reverse(T::MAX);
  const MAX: Self = Reverse(T::MIN);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn option_min_test() {
    let min = Option::<i32>::MIN;
    assert!(min <= None);
    assert!(min <= Some(i32::MIN));
  }

  #[test]
  fn option_max_test() {
    let max = Option::<i32>::MAX;
    assert!(max >= None);
    assert!(max >= Some(i32::MAX));
  }

  #[test]
  fn reverse_min_test() {
    let rev_min = Reverse::<i32>::MIN;
    assert!(rev_min <= Reverse(i32::MIN));
    assert!(rev_min <= Reverse(0));
    assert!(rev_min <= Reverse(i32::MAX));
  }

  #[test]
  fn reverse_max_test() {
    let rev_max = Reverse::<i32>::MAX;
    assert!(rev_max >= Reverse(i32::MIN));
    assert!(rev_max >= Reverse(0));
    assert!(rev_max >= Reverse(i32::MAX));
  }
}
