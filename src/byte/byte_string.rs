use crate::byte::{ByteChar, ByteStr};

use std::borrow::{Borrow, BorrowMut};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

/// An owned single-byte string.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteString(Vec<ByteChar>);

impl ByteString {
  /// Unwraps the vector of `ByteChar`.
  pub fn into_byte_chars(self) -> Vec<ByteChar> {
    self.0
  }

  /// Converts this `ByteString` to a `ByteStr`.
  pub fn as_byte_str(&self) -> &ByteStr {
    ByteStr::from_byte_chars(&self.0)
  }

  /// Converts this `ByteString` to a mutable `ByteStr`.
  pub fn as_mut_byte_str(&mut self) -> &mut ByteStr {
    ByteStr::from_byte_chars_mut(&mut self.0)
  }
}

impl From<Vec<ByteChar>> for ByteString {
  fn from(s: Vec<ByteChar>) -> ByteString {
    ByteString(s)
  }
}

impl Borrow<ByteStr> for ByteString {
  fn borrow(&self) -> &ByteStr {
    self.as_byte_str()
  }
}

impl BorrowMut<ByteStr> for ByteString {
  fn borrow_mut(&mut self) -> &mut ByteStr {
    self.as_mut_byte_str()
  }
}

impl Debug for ByteString {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    Debug::fmt(self.as_byte_str(), f)
  }
}

impl Display for ByteString {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    Display::fmt(self.as_byte_str(), f)
  }
}

impl Deref for ByteString {
  type Target = ByteStr;

  fn deref(&self) -> &ByteStr {
    ByteStr::from_byte_chars(&self.0)
  }
}

impl DerefMut for ByteString {
  fn deref_mut(&mut self) -> &mut ByteStr {
    ByteStr::from_byte_chars_mut(&mut self.0)
  }
}
