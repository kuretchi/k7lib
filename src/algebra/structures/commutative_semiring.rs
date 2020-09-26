use super::Semiring;
use crate::num::primitive::Int as PrimInt;

/// A commutative semiring.
///
/// # Laws
/// * Commutativity of multiplication: ∀`x` ∀`y` (`x.mul(&y)` = `y.mul(&x)`)
pub trait CommutativeSemiring: Semiring {}

impl CommutativeSemiring for () {}

impl<Int> CommutativeSemiring for Int where Int: PrimInt {}
