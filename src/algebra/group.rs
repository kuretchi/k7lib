use algebra::{Invertible, Monoid};

/// A group.
///
/// This trait is an alias of `Monoid + Invertible`, which has a blanket implementation.
pub trait Group: Monoid + Invertible {}

impl<T: Monoid + Invertible> Group for T {}
