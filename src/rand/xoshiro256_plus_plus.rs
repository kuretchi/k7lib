use super::Rng;

/// xoshiro256++ pseudorandom number generator.
///
/// # References
/// * [xoshiro/xoroshiro generators and the PRNG shootout][1] (Retrieved June 6, 2021)
///
/// [1]: http://prng.di.unimi.it/
#[derive(Clone, Copy, Debug)]
pub struct Xoshiro256PlusPlus {
  s: [u64; 4],
}

impl Default for Xoshiro256PlusPlus {
  fn default() -> Self {
    Self::seed_from_u32(0)
  }
}

impl Rng for Xoshiro256PlusPlus {
  fn seed_from_u32(seed: u32) -> Self {
    let mut x = seed.into();
    let mut s = [0; 4];
    for i in 0..4 {
      s[i] = split_mix64(&mut x);
    }
    Self { s }
  }

  fn next_u32(&mut self) -> u32 {
    self.next_u64() as _
  }

  fn next_u64(&mut self) -> u64 {
    let r = self.s[0].wrapping_add(self.s[3]).rotate_left(23).wrapping_add(self.s[0]);
    let t = self.s[1] << 17;
    self.s[2] ^= self.s[0];
    self.s[3] ^= self.s[1];
    self.s[1] ^= self.s[2];
    self.s[0] ^= self.s[3];
    self.s[2] ^= t;
    self.s[3] = self.s[3].rotate_left(45);
    r
  }
}

fn split_mix64(x: &mut u64) -> u64 {
  *x = x.wrapping_add(0x9e3779b97f4a7c15);
  let mut z = *x;
  z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
  z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
  z ^ (z >> 31)
}
