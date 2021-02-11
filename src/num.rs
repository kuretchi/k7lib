//! Utilities related to numeric types.

pub use self::div_ceil::*;
pub use self::div_floor::*;
pub use self::gcd::*;
pub use self::lcm::*;

mod div_ceil;
mod div_floor;
mod gcd;
mod lcm;
pub mod primitive;
