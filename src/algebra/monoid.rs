use algebra::{Semigroup, UnitalMagma};

/// A monoid.
///
/// This trait is an alias of `Semigroup + Unital`, which has a blanket implementation.
pub trait Monoid: Semigroup + UnitalMagma {}

impl<T: Semigroup + UnitalMagma> Monoid for T {}
