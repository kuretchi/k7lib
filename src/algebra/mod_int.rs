use crate::algebra::structures::{CommutativeSemiring, Ring, Semiring};
use crate::constant::Constant;
use crate::num::primitive::{Int as PrimInt, UnsignedInt as PrimUint};

use std::convert::TryFrom;
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
  _phantom: PhantomData<fn() -> Mod>,
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

impl<T, Int, Mod: Constant<Int>> From<T> for ModInt<Int, Mod>
where
  T: PrimInt + TryFrom<Int>,
  Int: PrimUint + TryFrom<T>,
{
  fn from(repr: T) -> Self {
    if let Ok(repr) = Int::try_from(repr) {
      return ModInt::new(repr);
    }
    // Here, `repr` < 0 or `Mod::get()` < `repr`
    if let Ok(mod_) = T::try_from(Mod::get()) {
      let repr = Int::try_from(repr.rem_euclid(mod_)).ok().unwrap();
      return ModInt::new_unchecked(repr);
    }
    // Here, `Mod::get()` > `T::MAX` (>= `repr`), and it implies `repr` < 0
    // For instance, `Mod::get() == 128u8` and `repr == -128i8`
    if let Some(repr_abs) = repr.checked_neg() {
      // `Mod::get()` > `T::MAX` = -`T::MIN` - 1 >= abs(`repr`) - 1 implies
      // abs(`repr`) <= `Mod::get()`, so this can never fail
      let repr_abs = Int::try_from(repr_abs).ok().unwrap();
      // `repr` < 0 implies abs(`repr`) > 0
      return ModInt::new_unchecked(Mod::get() - repr_abs);
    }
    // Here, `repr` = `T::MIN` = -`T::MAX` - 1
    debug_assert_eq!(repr, T::MIN);
    // Since `Mod::get()` > `T::MAX`, this can never fail
    let t_max = Int::try_from(T::MAX).ok().unwrap();
    // `Mod::get()` > `T::MAX` implies `T::MAX` + 1 <= `Mod::get()`
    ModInt::new_unchecked(Mod::get() - t_max - Int::ONE)
  }
}

impl<Int, Mod: Constant<Int>> ModInt<Int, Mod>
where
  Int: PrimUint,
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
  pub fn recip(self) -> Option<Self> {
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
      let recip = if swapped { s } else { Mod::get() - s };
      Some(ModInt::new_unchecked(recip))
    } else {
      None
    }
  }
}

impl<Int, Mod: Constant<Int>> Add for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  type Output = Self;

  #[allow(clippy::suspicious_arithmetic_impl)]
  fn add(self, rhs: Self) -> Self {
    let (mut sum, overflowed) = self.repr.overflowing_add(rhs.repr);
    if overflowed {
      // lhs + rhs - mod
      // = sum + 2^w - mod
      // = sum + !mod + 1
      sum += !Mod::get() + Int::ONE;
    } else if sum >= Mod::get() {
      sum -= Mod::get();
    }
    ModInt::new_unchecked(sum)
  }
}

impl<Int, Mod: Constant<Int>> Sub for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    self + -rhs
  }
}

impl<Int, Mod: Constant<Int>> Mul for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  type Output = Self;

  #[allow(clippy::suspicious_arithmetic_impl)]
  fn mul(self, rhs: Self) -> Self {
    if let Some(prod) = self.repr.checked_mul(rhs.repr) {
      return ModInt::new(prod);
    }
    let mut lhs = self;
    let mut rhs = rhs.repr;
    let mut acc = ModInt::new_unchecked(Int::ZERO);
    while rhs != Int::ZERO {
      if rhs & Int::ONE != Int::ZERO {
        // lhs * rhs = lhs + lhs * (rhs - 1)
        acc += lhs;
      }
      // lhs * rhs = (lhs * 2) * (rhs / 2)
      lhs += lhs;
      rhs >>= 1;
    }
    acc
  }
}

impl<Int, Mod: Constant<Int>> Div for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  type Output = Self;

  #[allow(clippy::suspicious_arithmetic_impl)]
  fn div(self, rhs: Self) -> Self {
    self * rhs.recip().expect("reciprocal does not exist")
  }
}

impl<Int, Mod: Constant<Int>> Neg for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  type Output = Self;

  fn neg(self) -> Self {
    if self.repr == Int::ZERO {
      self
    } else {
      ModInt::new_unchecked(Mod::get() - self.repr)
    }
  }
}

impl<Int, Mod: Constant<Int>> AddAssign for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl<Int, Mod: Constant<Int>> SubAssign for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl<Int, Mod: Constant<Int>> MulAssign for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl<Int, Mod: Constant<Int>> DivAssign for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl<Int, Mod: Constant<Int>> Sum for ModInt<Int, Mod>
where
  Int: PrimUint,
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
  Int: PrimUint,
{
  fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
    let mut acc = ModInt::new_unchecked(Int::ONE);
    for x in iter {
      acc *= x;
    }
    acc
  }
}

impl<Int, Mod: Constant<Int>> Semiring for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  fn add(&self, rhs: &Self) -> Self {
    *self + *rhs
  }
  fn mul(&self, rhs: &Self) -> Self {
    *self * *rhs
  }
  fn zero() -> Self {
    ModInt::new_unchecked(Int::ZERO)
  }
  fn one() -> Self {
    ModInt::new_unchecked(Int::ONE)
  }
}

impl<Int, Mod: Constant<Int>> CommutativeSemiring for ModInt<Int, Mod> where Int: PrimUint {}

impl<Int, Mod: Constant<Int>> Ring for ModInt<Int, Mod>
where
  Int: PrimUint,
{
  fn neg(&self) -> Self {
    -*self
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::constant;

  use quickcheck_macros::quickcheck;
  use std::convert::TryInto;

  type Uint = u8;
  type BigInt = i32;

  fn mod_repr(m: Uint, x: BigInt) -> Uint {
    x.rem_euclid(BigInt::from(m)).try_into().unwrap()
  }

  fn mod_add(m: Uint, x: Uint, y: Uint) -> Uint {
    (BigInt::from(x) + BigInt::from(y))
      .rem_euclid(BigInt::from(m))
      .try_into()
      .unwrap()
  }

  fn mod_sub(m: Uint, x: Uint, y: Uint) -> Uint {
    (BigInt::from(x) - BigInt::from(y))
      .rem_euclid(BigInt::from(m))
      .try_into()
      .unwrap()
  }

  fn mod_mul(m: Uint, x: Uint, y: Uint) -> Uint {
    (BigInt::from(x) * BigInt::from(y))
      .rem_euclid(BigInt::from(m))
      .try_into()
      .unwrap()
  }

  fn mod_neg(m: Uint, x: Uint) -> Uint {
    (-BigInt::from(x))
      .rem_euclid(BigInt::from(m))
      .try_into()
      .unwrap()
  }

  fn mod_sum(m: Uint, xs: impl IntoIterator<Item = Uint>) -> Uint {
    xs.into_iter().fold(0, |acc, x| mod_add(m, acc, x))
  }

  fn mod_product(m: Uint, xs: impl IntoIterator<Item = Uint>) -> Uint {
    xs.into_iter().fold(1, |acc, x| mod_mul(m, acc, x))
  }

  macro_rules! modulo {
    ($m:expr) => {
      if $m <= 1 {
        return;
      }
      constant! {
        static MOD: Uint = $m;
      }
    };
  }

  #[quickcheck]
  fn from_prop(m: Uint, x: BigInt) {
    modulo!(m);
    let a = ModInt::<_, MOD>::from(x);
    assert_eq!(a.repr(), mod_repr(m, x));
  }

  #[quickcheck]
  fn new_prop(m: Uint, x: Uint) {
    modulo!(m);
    let a = ModInt::<_, MOD>::new(x);
    assert_eq!(a.repr(), x % m);
  }

  #[quickcheck]
  fn recip_prop(m: Uint, x: Uint) {
    modulo!(m);
    let x = ModInt::<_, MOD>::new(x);
    match x.recip() {
      Some(x_recip) => assert_eq!((x * x_recip).repr(), 1),
      None => assert!((0..m).map(ModInt::new).all(|y| (x * y).repr() != 1)),
    }
  }

  #[quickcheck]
  fn add_prop(m: Uint, x: Uint, y: Uint) {
    modulo!(m);
    let a = ModInt::<_, MOD>::new(x) + ModInt::new(y);
    assert_eq!(a.repr(), mod_add(m, x, y));
  }

  #[quickcheck]
  fn sub_prop(m: Uint, x: Uint, y: Uint) {
    modulo!(m);
    let a = ModInt::<_, MOD>::new(x) - ModInt::new(y);
    assert_eq!(a.repr(), mod_sub(m, x, y));
  }

  #[quickcheck]
  fn mul_prop(m: Uint, x: Uint, y: Uint) {
    modulo!(m);
    let a = ModInt::<_, MOD>::new(x) * ModInt::new(y);
    assert_eq!(a.repr(), mod_mul(m, x, y));
  }

  #[quickcheck]
  fn neg_prop(m: Uint, x: Uint) {
    modulo!(m);
    let a = -ModInt::<_, MOD>::new(x);
    assert_eq!(a.repr(), mod_neg(m, x));
  }

  #[quickcheck]
  fn sum_prop(m: Uint, xs: Vec<Uint>) {
    modulo!(m);
    let a = xs.iter().copied().map(ModInt::new).sum::<ModInt<_, MOD>>();
    assert_eq!(a.repr(), mod_sum(m, xs));
  }

  #[quickcheck]
  fn product_prop(m: Uint, xs: Vec<Uint>) {
    modulo!(m);
    let a = xs
      .iter()
      .copied()
      .map(ModInt::new)
      .product::<ModInt<_, MOD>>();
    assert_eq!(a.repr(), mod_product(m, xs));
  }
}
