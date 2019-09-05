use algebra::{AssociativeMagma, UnitalMagma};

/// A monoid.
///
/// This trait is an alias of [`AssociativeMagma`] + [`UnitalMagma`].
///
/// [`AssociativeMagma`]: ./trait.AssociativeMagma.html
/// [`UnitalMagma`]: ./trait.UnitalMagma.html
pub trait Monoid: AssociativeMagma + UnitalMagma {}

impl<T: AssociativeMagma + UnitalMagma> Monoid for T {}
