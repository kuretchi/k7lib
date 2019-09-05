//! Algebraic structures.

pub use self::abelian_group::AbelianGroup;
pub use self::associative_magma::AssociativeMagma;
pub use self::commutative_magma::CommutativeMagma;
pub use self::group::Group;
pub use self::invertible_magma::InvertibleMagma;
pub use self::magma::Magma;
pub use self::monoid::Monoid;
pub use self::semigroup::Semigroup;
pub use self::unital_magma::UnitalMagma;

mod abelian_group;
mod associative_magma;
mod commutative_magma;
mod group;
mod invertible_magma;
mod magma;
mod monoid;
mod semigroup;
mod unital_magma;
