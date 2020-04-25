use std::ops::Neg;

/// A trait for primitive signed numeric types.
pub trait Signed: Sized + Neg<Output = Self> {}

/// A trait for primitive unsigned numeric types.
pub trait Unsigned {}

macro_rules! prim {
  ($($I:ty)*, $($U:ty)*) => {
    $(impl Signed for $I {})*
    $(impl Unsigned for $U {})*
  };
}

prim! { i8 i16 i32 i64 i128 isize, u8 u16 u32 u64 u128 usize }
