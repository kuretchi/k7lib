//! Algebraic structures.

pub use self::associative::AssociativeMagma;
pub use self::commutative::CommutativeMagma;
pub use self::group::Group;
pub use self::invertible::InvertibleMagma;
pub use self::magma::Magma;
pub use self::monoid::Monoid;
pub use self::semigroup::Semigroup;
pub use self::unital::UnitalMagma;

mod associative;
mod commutative;
mod group;
mod invertible;
mod magma;
mod monoid;
mod semigroup;
mod unital;
