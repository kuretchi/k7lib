use std::fmt::{self, Debug, Display, Formatter};

/// A single-byte character.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ByteChar(u8);

impl ByteChar {
  /// Creates a new `ByteChar` from a byte.
  pub fn new(c: u8) -> Self {
    ByteChar(c)
  }

  /// Unwraps the byte.
  pub fn into_byte(self) -> u8 {
    self.0
  }
}

impl Debug for ByteChar {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "b'{}'", self.0 as char)
  }
}

impl Display for ByteChar {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.0 as char)
  }
}
