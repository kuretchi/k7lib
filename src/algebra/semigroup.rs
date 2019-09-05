use algebra::{AssociativeMagma, Magma};

/// A semigroup.
///
/// This trait is an alias of `Magma + Associative`, which has a blanket implementation.
pub trait Semigroup: Magma + AssociativeMagma {}

impl<T: Magma + AssociativeMagma> Semigroup for T {}
