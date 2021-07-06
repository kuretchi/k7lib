use super::Rng;

/// A 64-bit multiplicative congruential generator (MCG).
///
/// # References
/// * [L’Ecuyer, P. (1999). Tables of linear congruential generators of different sizes and good lattice structure. Mathematics of Computation, 68(225), 249–260.][1]
///
/// [1]: https://doi.org/10.1090/S0025-5718-99-00996-5
#[derive(Clone, Copy, Debug)]
pub struct Mcg64(u64);

impl Default for Mcg64 {
  fn default() -> Self {
    Self(1)
  }
}

impl Rng for Mcg64 {
  fn seed_from_u32(seed: u32) -> Self {
    if seed == 0 {
      Self::default()
    } else {
      Self(seed.into())
    }
  }

  fn next_u32(&mut self) -> u32 {
    (self.next_u64() >> 32) as _
  }

  fn next_u64(&mut self) -> u64 {
    self.0 = self.0.wrapping_mul(1181783497276652981);
    self.0
  }
}
