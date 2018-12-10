use byte::{ByteChar, ByteString};

use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

/// A single-byte string slice.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteStr([ByteChar]);

macro_rules! cast {
  (mut $x:expr, $($T:ty)=>*) => {
    unsafe { &mut *($x $(as *mut $T)*) }
  };
  ($x:expr, $($T:ty)=>*) => {
    unsafe { &*($x $(as *const $T)*) }
  };
}

impl ByteStr {
  /// Converts a slice of bytes to a `ByteStr`.
  pub fn from_bytes(s: &[u8]) -> &Self {
    cast!(s, [u8] => [ByteChar] => ByteStr)
  }

  /// Converts a mutable slice of bytes to a mutable `ByteStr`.
  pub fn from_bytes_mut(s: &mut [u8]) -> &mut Self {
    cast!(mut s, [u8] => [ByteChar] => ByteStr)
  }

  /// Converts a slice of `ByteChar` to a `ByteStr`.
  pub fn from_byte_chars(s: &[ByteChar]) -> &Self {
    cast!(s, [ByteChar] => ByteStr)
  }

  /// Converts a mutable slice of `ByteChar` to a mutable `ByteStr`.
  pub fn from_byte_chars_mut(s: &mut [ByteChar]) -> &mut Self {
    cast!(mut s, [ByteChar] => ByteStr)
  }

  /// Converts this `ByteStr` to a slice of bytes.
  pub fn as_bytes(&self) -> &[u8] {
    cast!(self, ByteStr => [ByteChar] => [u8])
  }

  /// Converts this `ByteStr` to a mutable slice of bytes.
  pub fn as_bytes_mut(&mut self) -> &mut [u8] {
    cast!(mut self, ByteStr => [ByteChar] => [u8])
  }
}

impl ToOwned for ByteStr {
  type Owned = ByteString;

  fn to_owned(&self) -> ByteString {
    ByteString::from(self.0.to_owned())
  }
}

impl Debug for ByteStr {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "b\"")?;

    for &c in &self.0 {
      write!(f, "{}", c)?;
    }

    write!(f, "\"")
  }
}

impl Display for ByteStr {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    for &c in &self.0 {
      write!(f, "{}", c)?;
    }

    Ok(())
  }
}

impl Deref for ByteStr {
  type Target = [ByteChar];

  fn deref(&self) -> &[ByteChar] {
    &self.0
  }
}

impl DerefMut for ByteStr {
  fn deref_mut(&mut self) -> &mut [ByteChar] {
    &mut self.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut bytes = b"abcd".to_owned();
    let s = ByteStr::from_bytes_mut(&mut bytes);

    s.swap(1, 3); // b"adcb"
    s[0] = ByteChar::new(b'e'); // b"edcb"
    s.as_bytes_mut()[1] = b'f'; // b"efcb"

    assert_eq!(s.as_bytes(), b"efcb");
    assert_eq!(format!("{}", s), "efcb");
    assert_eq!(format!("{:?}", s), "b\"efcb\"");
  }
}
