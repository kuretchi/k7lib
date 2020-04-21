use super::AssociativeMagma;

/// A semigroup.
///
/// This trait is an alias of [`AssociativeMagma`].
///
/// [`AssociativeMagma`]: ./trait.AssociativeMagma.html
pub trait Semigroup: AssociativeMagma {}

impl<T: AssociativeMagma> Semigroup for T {}
