use algebra::{Semigroup, Unital};

/// A monoid.
///
/// This trait is an alias of `Semigroup + Unital`, which has a blanket implementation.
pub trait Monoid: Semigroup + Unital {}

impl<T: Semigroup + Unital> Monoid for T {}
