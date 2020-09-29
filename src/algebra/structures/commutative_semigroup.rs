use super::Semigroup;

/// A commutative semigroup.
///
/// # Laws
/// * Commutativity: ∀`x` ∀`y` (`x.op(&y)` = `y.op(&x)`)
pub trait CommutativeSemigroup: Semigroup {}

impl CommutativeSemigroup for () {}

impl<T> CommutativeSemigroup for Option<T> where T: CommutativeSemigroup {}
