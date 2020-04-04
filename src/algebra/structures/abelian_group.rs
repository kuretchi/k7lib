use super::{CommutativeMagma, Group};

/// An abelian group.
///
/// This trait is an alias of [`Group`] + [`CommutativeMagma`].
///
/// [`Group`]: ./trait.Group.html
/// [`CommutativeMagma`]: ./trait.CommutativeMagma.html
pub trait AbelianGroup: Group + CommutativeMagma {}

impl<T: Group + CommutativeMagma> AbelianGroup for T {}
