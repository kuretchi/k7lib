use super::Magma;

/// A commutative magma.
///
/// # Laws
/// * Commutativity: ∀`x` ∀`y` (`x.op(&y)` = `y.op(&x)`)
pub trait CommutativeMagma: Magma {}

impl CommutativeMagma for () {}

impl<T> CommutativeMagma for Option<T> where T: CommutativeMagma {}
