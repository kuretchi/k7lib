//! Utilities related to numeric types.

pub use self::div_ceil::*;
pub use self::div_floor::*;
pub use self::gcd::*;
pub use self::lcm::*;
pub use self::midpoint::*;

mod div_ceil;
mod div_floor;
mod gcd;
mod lcm;
mod midpoint;
pub mod primitive;
