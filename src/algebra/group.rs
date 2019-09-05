use algebra::{InvertibleMagma, Monoid};

/// A group.
///
/// This trait is an alias of `Monoid + Invertible`, which has a blanket implementation.
pub trait Group: Monoid + InvertibleMagma {}

impl<T: Monoid + InvertibleMagma> Group for T {}
