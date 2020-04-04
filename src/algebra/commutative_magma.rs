use crate::algebra::Magma;

/// A commutative magma.
///
/// # Laws
/// * Commutativity: ∀`x` ∀`y` (`x.op(&y)` = `y.op(&x)`)
pub trait CommutativeMagma: Magma {}
