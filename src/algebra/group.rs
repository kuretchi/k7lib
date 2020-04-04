use crate::algebra::{AssociativeMagma, InvertibleMagma, UnitalMagma};

/// A group.
///
/// This trait is an alias of [`AssociativeMagma`] + [`UnitalMagma`] + [`InvertibleMagma`].
///
/// [`AssociativeMagma`]: ./trait.AssociativeMagma.html
/// [`UnitalMagma`]: ./trait.UnitalMagma.html
/// [`InvertibleMagma`]: ./trait.InvertibleMagma.html
pub trait Group: AssociativeMagma + UnitalMagma + InvertibleMagma {}

impl<T: AssociativeMagma + UnitalMagma + InvertibleMagma> Group for T {}
