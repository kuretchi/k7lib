//! Algebraic structures.

pub use self::abelian_group::*;
pub use self::associative_magma::*;
pub use self::commutative_magma::*;
pub use self::group::*;
pub use self::invertible_magma::*;
pub use self::magma::*;
pub use self::monoid::*;
pub use self::semigroup::*;
pub use self::unital_magma::*;

mod abelian_group;
mod associative_magma;
mod commutative_magma;
mod group;
mod invertible_magma;
mod magma;
mod monoid;
mod semigroup;
mod unital_magma;
