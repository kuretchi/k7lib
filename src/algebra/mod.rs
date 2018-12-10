//! Algebraic structures.

pub use self::associative::Associative;
pub use self::commutative::Commutative;
pub use self::group::Group;
pub use self::invertible::Invertible;
pub use self::magma::Magma;
pub use self::monoid::Monoid;
pub use self::semigroup::Semigroup;
pub use self::unital::Unital;

mod associative;
mod commutative;
mod group;
mod invertible;
mod magma;
mod monoid;
mod semigroup;
mod unital;
