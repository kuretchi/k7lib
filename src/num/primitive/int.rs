use crate::cmp::Bounded;

use std::fmt::{Binary, Debug, Display, LowerHex, Octal, UpperHex};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::ops::Not;
use std::str::FromStr;

macro_rules! int {
  ($($Op:ident $OpAssign:ident $Rhs:ident)*) => {
    use std::ops::{$($Op, $OpAssign),*};

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
      + Hash
      + Debug
      + Display
      + Binary
      + Octal
      + LowerHex
      + UpperHex
      + FromStr
      $(
        + $Op<$Rhs, Output = Self>
        + for<'a> $Op<&'a $Rhs, Output = Self>
        + $OpAssign<$Rhs>
        + for<'a> $OpAssign<&'a $Rhs>
      )*
      + Not
      + Sum
      + for<'a> Sum<&'a Self>
      + Product
      + for<'a> Product<&'a Self>
      + Bounded
    {
      /// The constant value `0`.
      const ZERO: Self;

      /// The constant value `1`.
      const ONE: Self;
    }
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
  Shr ShrAssign u32
}

macro_rules! prim {
  ($($T:ty)*) => {$(
    impl Int for $T {
      const ZERO: Self = 0;
      const ONE: Self = 1;
    }
  )*};
}

prim! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
