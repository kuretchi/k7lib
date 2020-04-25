use crate::constant::Constant;
use crate::num::primitive::{Int as PrimInt, Unsigned};

use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{Product, Sum};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A ring of integers modulo n (ℤ/nℤ).
///
/// `Mod` must be greater than 1.
///
/// # Examples
///
/// ```
/// # use spella::algebra::ModInt;
/// # use spella::constant;
/// constant! {
///   const MOD: u32 = 4;
/// }
///
/// let x = ModInt::<u32, MOD>::new(17);
/// let y = ModInt::new(3);
///
/// assert_eq!(x.repr(), 1);
/// assert_eq!(y.repr(), 3);
/// assert_eq!((x / y).repr(), 3); // (x / y) * y = 3 * 3 = 9 = 1 = x
/// ```
pub struct ModInt<Int, Mod> {
  _phantom: PhantomData<fn() -> (Int, Mod)>,
  repr: Int,
}

impl<Int, Mod> Clone for ModInt<Int, Mod>
where
  Int: Clone,
{
  fn clone(&self) -> Self {
    ModInt {
      _phantom: PhantomData,
      repr: self.repr.clone(),
    }
  }
}

impl<Int, Mod> Copy for ModInt<Int, Mod> where Int: Copy {}

impl<Int, Mod> PartialEq for ModInt<Int, Mod>
where
  Int: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.repr.eq(&other.repr)
  }
}

impl<Int, Mod> Eq for ModInt<Int, Mod> where Int: Eq {}

impl<Int, Mod> Default for ModInt<Int, Mod>
where
  Int: Default,
{
  fn default() -> Self {
    ModInt {
      _phantom: PhantomData,
      repr: Int::default(),
    }
  }
}

impl<Int, Mod> Hash for ModInt<Int, Mod>
where
  Int: Hash,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.repr.hash(state);
  }
}

impl<Int, Mod> Debug for ModInt<Int, Mod>
where
  Int: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&self.repr, f)
  }
}

impl<Int, Mod> Display for ModInt<Int, Mod>
where
  Int: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&self.repr, f)
  }
}

impl<Int, Mod: Constant<Int>> ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  fn new_unchecked(repr: Int) -> Self {
    debug_assert!(Mod::get() > Int::ONE, "modulo must be greater than 1");
    debug_assert!(repr < Mod::get());
    ModInt {
      _phantom: PhantomData,
      repr,
    }
  }

  /// Creates a new `ModInt` with the given representative.
  pub fn new(repr: Int) -> Self {
    ModInt::new_unchecked(repr % Mod::get())
  }

  /// Returns the canonical representative.
  pub fn repr(self) -> Int {
    self.repr
  }

  /// Returns the multiplicative inverse if it exists.
  ///
  /// # Time complexity
  /// O(log `self.repr()`)
  pub fn inverse(self) -> Option<Self> {
    // The extended Euclidean algorithm
    let mut a = Mod::get();
    let mut b = self.repr;
    let mut s = Int::ZERO;
    let mut t = Int::ONE;
    let mut swapped = false;

    while b != Int::ZERO {
      // `a` ≡ `s * self.repr` and `b` ≡ -`t * self.repr` (if `swapped`)
      // `a` ≡ -`s * self.repr` and `b` ≡ `t * self.repr` (if `!swapped`)
      let q = a / b;
      a -= q * b;
      s += q * t;
      mem::swap(&mut a, &mut b);
      mem::swap(&mut s, &mut t);
      swapped = !swapped;
    }

    // Here `a` is the gcd of `Mod::get()` and `self.repr`
    if a == Int::ONE {
      // 1 ≡ `s * self.repr` (if `swapped`)
      // 1 ≡ -`s * self.repr` (if `!swapped`)
      let inv = if swapped { s } else { Mod::get() - s };
      Some(ModInt::new_unchecked(inv))
    } else {
      None
    }
  }
}

impl<Int, Mod: Constant<Int>> Add for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self {
    self += rhs;
    self
  }
}

impl<Int, Mod: Constant<Int>> Sub for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  type Output = Self;

  fn sub(mut self, rhs: Self) -> Self {
    self -= rhs;
    self
  }
}

impl<Int, Mod: Constant<Int>> Mul for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    ModInt::new(self.repr * rhs.repr)
  }
}

impl<Int, Mod: Constant<Int>> Div for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  type Output = Self;

  fn div(self, rhs: Self) -> Self {
    self * rhs.inverse().expect("inverse does not exist")
  }
}

impl<Int, Mod: Constant<Int>> Neg for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  type Output = Self;

  fn neg(self) -> Self {
    ModInt::new_unchecked(Int::ZERO) - self
  }
}

impl<Int, Mod: Constant<Int>> AddAssign for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  fn add_assign(&mut self, rhs: Self) {
    self.repr += rhs.repr;
    if self.repr >= Mod::get() {
      self.repr -= Mod::get();
    }
  }
}

impl<Int, Mod: Constant<Int>> SubAssign for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  fn sub_assign(&mut self, rhs: Self) {
    if self.repr < rhs.repr {
      self.repr += Mod::get();
    }
    self.repr -= rhs.repr;
  }
}

impl<Int, Mod: Constant<Int>> MulAssign for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl<Int, Mod: Constant<Int>> DivAssign for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl<Int, Mod: Constant<Int>> Sum for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    let mut acc = ModInt::new_unchecked(Int::ZERO);
    for x in iter {
      acc += x;
    }
    acc
  }
}

impl<Int, Mod: Constant<Int>> Product for ModInt<Int, Mod>
where
  Int: PrimInt + Unsigned,
{
  fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
    let mut acc = ModInt::new(Int::ONE);
    for x in iter {
      acc *= x;
    }
    acc
  }
}
