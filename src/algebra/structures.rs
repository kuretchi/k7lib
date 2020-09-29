//! Algebraic structures.

pub use self::commutative_semigroup::*;
pub use self::commutative_semiring::*;
pub use self::group::*;
pub use self::monoid::*;
pub use self::ring::*;
pub use self::semigroup::*;
pub use self::semiring::*;
pub use self::systems::*;

mod commutative_semigroup;
mod commutative_semiring;
mod group;
mod monoid;
mod ring;
mod semigroup;
mod semiring;
mod systems;
