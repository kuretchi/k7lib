//! Algebraic structures.

pub use self::commutative_semigroup::*;
pub use self::commutative_semiring::*;
pub use self::group::*;
pub use self::monoid::*;
pub use self::ring::*;
pub use self::semigroup::*;
pub use self::semiring::*;

mod commutative_semigroup;
mod commutative_semiring;
mod group;
mod monoid;
mod ring;
mod semigroup;
mod semiring;

pub use self::all::*;
pub use self::any::*;
pub use self::concat::*;
pub use self::first::*;
pub use self::last::*;
pub use self::max::*;
pub use self::min::*;
pub use self::product::*;
pub use self::sum::*;

mod all;
mod any;
mod concat;
mod first;
mod last;
mod max;
mod min;
mod product;
mod sum;
