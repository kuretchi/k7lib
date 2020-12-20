use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

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

impl FromStr for ByteChar {
  type Err = ParseByteCharError;

  fn from_str(s: &str) -> Result<ByteChar, ParseByteCharError> {
    match s.as_bytes().len() {
      1 => Ok(ByteChar(s.as_bytes()[0])),
      0 => Err(ParseByteCharErrorKind::EmptyStr.into()),
      _ => Err(ParseByteCharErrorKind::TooManyBytes.into()),
    }
  }
}

/// An error which can be returned when parsing a `ByteChar`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ParseByteCharError {
  kind: ParseByteCharErrorKind,
}

impl Display for ParseByteCharError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    f.write_str(match self.kind {
      ParseByteCharErrorKind::EmptyStr => "empty string",
      ParseByteCharErrorKind::TooManyBytes => "too many bytes",
    })
  }
}

impl Error for ParseByteCharError {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ParseByteCharErrorKind {
  EmptyStr,
  TooManyBytes,
}

impl From<ParseByteCharErrorKind> for ParseByteCharError {
  fn from(kind: ParseByteCharErrorKind) -> ParseByteCharError {
    ParseByteCharError { kind }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_str_test() {
    assert_eq!(ByteChar::from_str("a"), Ok(ByteChar(b'a')));
    assert_eq!(ByteChar::from_str("A"), Ok(ByteChar(b'A')));
    assert_eq!(ByteChar::from_str("0"), Ok(ByteChar(b'0')));
    assert_eq!(ByteChar::from_str(" "), Ok(ByteChar(b' ')));
    assert_eq!(ByteChar::from_str("!"), Ok(ByteChar(b'!')));
  }

  #[test]
  fn from_str_test_empty_str() {
    assert_eq!(
      ByteChar::from_str(""),
      Err(ParseByteCharError { kind: ParseByteCharErrorKind::EmptyStr })
    );
  }

  #[test]
  fn from_str_test_too_many_bytes() {
    assert_eq!(
      ByteChar::from_str("aa"),
      Err(ParseByteCharError { kind: ParseByteCharErrorKind::TooManyBytes })
    );
    assert_eq!(
      ByteChar::from_str("あ"),
      Err(ParseByteCharError { kind: ParseByteCharErrorKind::TooManyBytes })
    );
  }
}
