use crate::cmp::Bounded;
use std::{fmt, hash, iter, num::ParseIntError, ops, str};

// for cargo-simple-bundler
#[cfg(any())]
pub trait Int {}

macro_rules! int {
  ($($Op:ident $OpAssign:ident $Rhs:ty)*; $($func:tt)*) => {
    /// A trait for primitive integer types.
    pub trait Int:
      Sized
      + Clone
      + Copy
      + PartialEq
      + Eq
      + PartialOrd
      + Ord
      + Default
      + hash::Hash
      + fmt::Debug
      + fmt::Display
      + fmt::Binary
      + fmt::Octal
      + fmt::LowerHex
      + fmt::UpperHex
      + str::FromStr
      $(
        + ops::$Op<$Rhs, Output = Self>
        + for<'a> ops::$Op<&'a $Rhs, Output = Self>
        + ops::$OpAssign<$Rhs>
        + for<'a> ops::$OpAssign<&'a $Rhs>
      )*
      + ops::Not<Output = Self>
      + iter::Sum
      + for<'a> iter::Sum<&'a Self>
      + iter::Product
      + for<'a> iter::Product<&'a Self>
      + Bounded
    {
      /// The constant value `0`.
      const ZERO: Self;
      /// The constant value `1`.
      const ONE: Self;

      $($func)*
    }

    impls! {
      Int => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize;
      const ZERO: Self = 0;
      const ONE: Self = 1;
      $($func)*
    }
  };
}

// for cargo-simple-bundler
#[cfg(any())]
pub trait SignedInt {}

macro_rules! signed_int {
  ($($func:tt)*) => {
    /// A trait for primitive signed integer types.
    pub trait SignedInt: Int + ops::Neg<Output = Self> {
      $($func)*
    }
    impls! { SignedInt => i8, i16, i32, i64, i128, isize; $($func)* }
  };
}

// for cargo-simple-bundler
#[cfg(any())]
pub trait UnsignedInt {}

macro_rules! unsigned_int {
  ($($func:tt)*) => {
    /// A trait for primitive unsigned integer types.
    pub trait UnsignedInt: Int {
      $($func)*
    }
    impls! { UnsignedInt => u8, u16, u32, u64, u128, usize; $($func)* }
  };
}

macro_rules! impls {
  ($Trait:ty => ; $($item:tt)*) => {};
  ($Trait:ty => $T:ty $(, $U:ty)*; $($item:tt)*) => {
    impl $Trait for $T {
      items! { $($item)* }
    }
    impls! { $Trait => $($U),*; $($item)* }
  };
}

macro_rules! items {
  () => {};
  (const $CONST:ident: $T:ty = $val:expr; $($rest:tt)*) => {
    const $CONST: $T = $val;
    items! { $($rest)* }
  };
  (fn $func:ident(self $(, $param:ident: $ParamT:ty)*) -> $RetT:ty; $($rest:tt)*) => {
    #[deny(unconditional_recursion)]
    fn $func(self $(, $param: $ParamT)*) -> $RetT {
      Self::$func(self $(, $param)*)
    }
    items! { $($rest)* }
  };
  (fn $func:ident($($param:ident: $ParamT:ty),*) -> $RetT:ty; $($rest:tt)*) => {
    #[deny(unconditional_recursion)]
    fn $func($($param: $ParamT),*) -> $RetT {
      Self::$func($($param),*)
    }
    items! { $($rest)* }
  };
}

int! {
  Add AddAssign Self
  Sub SubAssign Self
  Mul MulAssign Self
  Div DivAssign Self
  Rem RemAssign Self
  BitAnd BitAndAssign Self
  BitOr BitOrAssign Self
  BitXor BitXorAssign Self
  Shl ShlAssign u32
  Shr ShrAssign u32;

  fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
  fn count_ones(self) -> u32;
  fn count_zeros(self) -> u32;
  fn leading_zeros(self) -> u32;
  fn trailing_zeros(self) -> u32;
  fn rotate_left(self, n: u32) -> Self;
  fn rotate_right(self, n: u32) -> Self;
  fn swap_bytes(self) -> Self;
  fn reverse_bits(self) -> Self;
  fn checked_add(self, rhs: Self) -> Option<Self>;
  fn checked_sub(self, rhs: Self) -> Option<Self>;
  fn checked_mul(self, rhs: Self) -> Option<Self>;
  fn checked_div(self, rhs: Self) -> Option<Self>;
  fn checked_div_euclid(self, rhs: Self) -> Option<Self>;
  fn checked_rem(self, rhs: Self) -> Option<Self>;
  fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;
  fn checked_neg(self) -> Option<Self>;
  fn checked_shl(self, rhs: u32) -> Option<Self>;
  fn checked_shr(self, rhs: u32) -> Option<Self>;
  fn checked_pow(self, exp: u32) -> Option<Self>;
  fn saturating_add(self, rhs: Self) -> Self;
  fn saturating_sub(self, rhs: Self) -> Self;
  fn saturating_mul(self, rhs: Self) -> Self;
  fn saturating_pow(self, exp: u32) -> Self;
  fn wrapping_add(self, rhs: Self) -> Self;
  fn wrapping_sub(self, rhs: Self) -> Self;
  fn wrapping_mul(self, rhs: Self) -> Self;
  fn wrapping_div(self, rhs: Self) -> Self;
  fn wrapping_div_euclid(self, rhs: Self) -> Self;
  fn wrapping_rem(self, rhs: Self) -> Self;
  fn wrapping_rem_euclid(self, rhs: Self) -> Self;
  fn wrapping_neg(self) -> Self;
  fn wrapping_shl(self, rhs: u32) -> Self;
  fn wrapping_shr(self, rhs: u32) -> Self;
  fn wrapping_pow(self, exp: u32) -> Self;
  fn overflowing_add(self, rhs: Self) -> (Self, bool);
  fn overflowing_sub(self, rhs: Self) -> (Self, bool);
  fn overflowing_mul(self, rhs: Self) -> (Self, bool);
  fn overflowing_div(self, rhs: Self) -> (Self, bool);
  fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);
  fn overflowing_rem(self, rhs: Self) -> (Self, bool);
  fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);
  fn overflowing_neg(self) -> (Self, bool);
  fn overflowing_shl(self, rhs: u32) -> (Self, bool);
  fn overflowing_shr(self, rhs: u32) -> (Self, bool);
  fn overflowing_pow(self, exp: u32) -> (Self, bool);
  fn pow(self, exp: u32) -> Self;
  fn div_euclid(self, rhs: Self) -> Self;
  fn rem_euclid(self, rhs: Self) -> Self;
}

signed_int! {
  fn checked_abs(self) -> Option<Self>;
  fn wrapping_abs(self) -> Self;
  fn overflowing_abs(self) -> (Self, bool);
  fn abs(self) -> Self;
  fn signum(self) -> Self;
  fn is_positive(self) -> bool;
  fn is_negative(self) -> bool;
}

unsigned_int! {
  fn is_power_of_two(self) -> bool;
  fn next_power_of_two(self) -> Self;
  fn checked_next_power_of_two(self) -> Option<Self>;
}
