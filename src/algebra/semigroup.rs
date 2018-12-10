use algebra::{Associative, Magma};

/// A semigroup.
///
/// This trait is an alias of `Magma + Associative`, which has a blanket implementation.
pub trait Semigroup: Magma + Associative {}

impl<T: Magma + Associative> Semigroup for T {}
