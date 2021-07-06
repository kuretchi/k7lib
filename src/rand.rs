//! Random number generation.

pub use mcg64::*;
pub use xoshiro256_plus_plus::*;

mod mcg64;
mod xoshiro256_plus_plus;

/// Random number generators (RNGs).
pub trait Rng {
  /// Creates a new RNG from a 32-bit seed.
  fn seed_from_u32(seed: u32) -> Self;

  /// Generates a uniformly distributed random 32-bit integer.
  fn next_u32(&mut self) -> u32;

  /// Generates a uniformly distributed random 64-bit integer.
  fn next_u64(&mut self) -> u64;

  /// Generates a uniformly distributed random value of type `T`.
  fn gen<T>(&mut self) -> T
  where
    T: Sample,
  {
    T::sample(self)
  }
}

/// Generating of a uniformly distributed random value.
pub trait Sample: Sized {
  /// Generates a uniformly distributed random value of this type.
  fn sample<R>(rng: &mut R) -> Self
  where
    R: ?Sized + Rng;
}

macro_rules! impl_sample {
  ($($(#[$m:meta])* $t:ty)* |$r:ident| $x:expr) => {$(
    $(#[$m])*
    impl Sample for $t {
      fn sample<R>($r: &mut R) -> Self
      where
        R: ?Sized + Rng,
      {
        ($x) as $t
      }
    }
  )*};
}

impl_sample! {
  ()
  |_rng| ()
}

impl_sample! {
  bool
  // Checks the most significant bit
  |rng| (rng.next_u32() as i32).is_negative()
}

impl_sample! {
  i8 i16 i32 u8 u16 u32
  |rng| rng.next_u32()
}

impl_sample! {
  i64 u64
  |rng| rng.next_u64()
}

impl_sample! {
  i128 u128
  |rng| u128::from(rng.next_u64()) << 64 | u128::from(rng.next_u64())
}

impl_sample! {
  isize usize
  |rng| {
    #[cfg(target_pointer_width = "32")]
    let r = rng.next_u32();
    #[cfg(target_pointer_width = "64")]
    let r = rng.next_u64();
    r
  }
}

impl_sample! {
  /// An uniform distribution in range $[0, 1)$.
  f32
  // 0_01111111_????..
  // ^ ^^^^^^^^ ^^^^^^
  // +-|--------|----- sign = 0
  //   +--------|----- exponent = 127
  //            +----- fraction = 0..1 (random)
  // = (-1)^sign * 2^(exponent - 127) * (1 + fraction)
  // = 1 + fraction
  |rng| {
    const FRACTION_BITS: u32 = 23;
    f32::from_bits((0b0_01111111 << FRACTION_BITS) | (rng.next_u32() >> 32 - FRACTION_BITS)) - 1.
  }
}

impl_sample! {
  /// An uniform distribution in range $[0, 1)$.
  f64
  |rng| {
    const FRACTION_BITS: u32 = 52;
    f64::from_bits((0b0_01111111111 << FRACTION_BITS) | (rng.next_u64() >> 64 - FRACTION_BITS)) - 1.
  }
}

/// Returns the system time's subsec as entropy for PRNGs.
pub fn entropy_from_time() -> u32 {
  use std::time::SystemTime;

  SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("system time before the Unix epoch")
    .subsec_nanos()
}
