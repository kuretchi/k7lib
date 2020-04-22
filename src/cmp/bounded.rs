use std::cmp::Reverse;

/// Trait for types that have min/max values.
pub trait Bounded: Ord {
  fn min_value() -> Self;
  fn max_value() -> Self;
}

macro_rules! prim {
  ($($T:ty)*) => {$(
    impl Bounded for $T {
      fn min_value() -> Self {
        <$T>::min_value()
      }

      fn max_value() -> Self {
        <$T>::max_value()
      }
    }
  )*};
}

prim! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

impl Bounded for bool {
  fn min_value() -> Self {
    false
  }

  fn max_value() -> Self {
    true
  }
}

#[allow(clippy::unused_unit)]
impl Bounded for () {
  fn min_value() -> Self {
    ()
  }

  fn max_value() -> Self {
    ()
  }
}

impl<T> Bounded for Option<T>
where
  T: Bounded,
{
  fn min_value() -> Self {
    None
  }

  fn max_value() -> Self {
    Some(T::max_value())
  }
}

impl<T> Bounded for Reverse<T>
where
  T: Bounded,
{
  fn min_value() -> Self {
    Reverse(T::max_value())
  }

  fn max_value() -> Self {
    Reverse(T::min_value())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn option_min_test() {
    let min = Option::<i32>::min_value();
    assert!(min <= None);
    assert!(min <= Some(i32::min_value()));
  }

  #[test]
  fn option_max_test() {
    let max = Option::<i32>::max_value();
    assert!(max >= None);
    assert!(max >= Some(i32::max_value()));
  }

  #[test]
  fn reverse_min_test() {
    let rev_min = Reverse::<i32>::min_value();
    assert!(rev_min <= Reverse(i32::min_value()));
    assert!(rev_min <= Reverse(0));
    assert!(rev_min <= Reverse(i32::max_value()));
  }

  #[test]
  fn reverse_max_test() {
    let rev_max = Reverse::<i32>::max_value();
    assert!(rev_max >= Reverse(i32::min_value()));
    assert!(rev_max >= Reverse(0));
    assert!(rev_max >= Reverse(i32::max_value()));
  }
}
