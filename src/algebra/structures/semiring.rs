use crate::num::primitive::Int as PrimInt;
#[allow(unused_imports)]
use crate::utils::for_each_tuple; // for cargo-simple-bundler

/// A semiring.
///
/// # Laws
/// * It forms a commutative monoid under addition.
///   * Associativity: ∀`x` ∀`y` ∀`z` (`x.add(&y).add(&z)` = `x.add(&y.add(&z))`)
///   * Identity: ∀`x` (`x.add(&Self::zero())` = `Self::zero().add(&x)` = `x`)
///   * Commutativity: ∀`x` ∀`y` (`x.add(&y)` = `y.add(&x)`)
/// * It forms a monoid under multiplication.
///   * Associativity: ∀`x` ∀`y` ∀`z` (`x.mul(&y).mul(&z)` = `x.mul(&y.mul(&z))`)
///   * Identity: ∀`x` (`x.mul(&Self::one())` = `Self::one().mul(&x)` = `x`)
/// * Distributivity:
///   * Left: ∀`x` ∀`y` ∀`z` (`x.mul(&y.add(&z))` = `x.mul(&y).add(&x.mul(&z))`)
///   * Right: ∀`x` ∀`y` ∀`z` (`x.add(&y).mul(&z)` = `x.mul(&z).add(&y.mul(&z))`)
/// * ∀`x` (`x.mul(&Self::zero())` = `Self::zero().mul(&x)` = `Self::zero()`)
pub trait Semiring: Clone {
  /// Performs addition.
  fn add(&self, rhs: &Self) -> Self;

  /// Performs multiplication.
  fn mul(&self, rhs: &Self) -> Self;

  /// Returns the additive identity.
  fn zero() -> Self;

  /// Returns the multiplicative identity.
  fn one() -> Self;
}

#[allow(clippy::unused_unit)]
/// The trivial semiring.
impl Semiring for () {
  fn add(&self, _rhs: &Self) -> Self {
    ()
  }
  fn mul(&self, _rhs: &Self) -> Self {
    ()
  }
  fn zero() -> Self {
    ()
  }
  fn one() -> Self {
    ()
  }
}

/// Usual addition and multiplication over integers.
impl<Int> Semiring for Int
where
  Int: PrimInt,
{
  fn add(&self, rhs: &Self) -> Self {
    *self + *rhs
  }
  fn mul(&self, rhs: &Self) -> Self {
    *self * *rhs
  }
  fn zero() -> Self {
    Int::ZERO
  }
  fn one() -> Self {
    Int::ONE
  }
}

macro_rules! impl_for_tuple {
  ($($i:tt: $T:ident,)*) => {
    impl<$($T),*> Semiring for ($($T,)*)
    where
      $($T: Semiring,)*
    {
      fn add(&self, rhs: &Self) -> Self {
        ($(self.$i.add(&rhs.$i),)*)
      }
      fn mul(&self, rhs: &Self) -> Self {
        ($(self.$i.mul(&rhs.$i),)*)
      }
      fn zero() -> Self {
        ($(<$T>::zero(),)*)
      }
      fn one() -> Self {
        ($(<$T>::one(),)*)
      }
    }
  };
}

for_each_tuple! { impl_for_tuple }
