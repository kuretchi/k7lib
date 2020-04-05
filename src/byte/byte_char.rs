use std::fmt::{self, Debug, Display, Formatter};

/// A single-byte character.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ByteChar(pub u8);

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
