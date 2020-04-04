use crate::byte::{ByteChar, ByteStr, ByteString};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::{self, FromStr, Utf8Error};

/// A trait for types parsing a `ByteStr`.
pub trait FromByteStr: Sized {
  /// The type of error which can be returned from parsing.
  type Err;

  /// Parses a `ByteStr` and returns a value of this type.
  fn from_byte_str(s: &ByteStr) -> Result<Self, Self::Err>;
}

/// An error which can be returned when parsing a `ByteChar`.
#[derive(Debug)]
pub struct ParseByteCharError(ParseByteCharErrorKind);

#[derive(Debug)]
enum ParseByteCharErrorKind {
  EmptyByteStr,
  TooManyByteChars,
}

impl Display for ParseByteCharError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use self::ParseByteCharErrorKind::*;

    f.write_str(match self.0 {
      EmptyByteStr => "empty `ByteStr`",
      TooManyByteChars => "too many `ByteChar`s",
    })
  }
}

impl Error for ParseByteCharError {}

impl FromByteStr for ByteChar {
  type Err = ParseByteCharError;

  fn from_byte_str(s: &ByteStr) -> Result<Self, Self::Err> {
    use self::ParseByteCharErrorKind::*;

    match s.len() {
      1 => Ok(unsafe { *s.get_unchecked(0) }),
      0 => Err(ParseByteCharError(EmptyByteStr)),
      _ => Err(ParseByteCharError(TooManyByteChars)),
    }
  }
}

/// An error which can be returned when parsing a `ByteString`.
///
/// This is an empty `enum`, so it will never actually exist.
/// This is because `ByteString::from_byte_str` will never fail.
#[derive(Debug)]
pub enum ParseByteStringError {}

impl Display for ParseByteStringError {
  fn fmt(&self, _: &mut Formatter) -> fmt::Result {
    match *self {}
  }
}

impl Error for ParseByteStringError {}

impl FromByteStr for ByteString {
  type Err = ParseByteStringError;

  fn from_byte_str(s: &ByteStr) -> Result<Self, Self::Err> {
    Ok(ByteString::from(s.to_vec()))
  }
}

/// An error which can be returned when parsing a value of a type which implements `FromStr`.
pub struct ParseFromStrError<T: FromStr>(ParseFromStrErrorKind<T>);

enum ParseFromStrErrorKind<T: FromStr> {
  Utf8Error(Utf8Error),
  FromStrError(T::Err),
}

impl<T: FromStr> Debug for ParseFromStrError<T>
where
  T::Err: Debug,
{
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use self::ParseFromStrErrorKind::*;

    match self.0 {
      Utf8Error(ref err) => f.debug_tuple("Utf8Error").field(err).finish(),
      FromStrError(ref err) => f.debug_tuple("FromStrError").field(err).finish(),
    }
  }
}

impl<T: FromStr> Display for ParseFromStrError<T>
where
  T::Err: Display,
{
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use self::ParseFromStrErrorKind::*;

    match self.0 {
      Utf8Error(ref err) => write!(f, "{}", err),
      FromStrError(ref err) => write!(f, "{}", err),
    }
  }
}

impl<T: FromStr> Error for ParseFromStrError<T> where T::Err: Debug + Display {}

impl<T: FromStr> FromByteStr for T {
  type Err = ParseFromStrError<T>;

  fn from_byte_str(s: &ByteStr) -> Result<T, Self::Err> {
    use self::ParseFromStrErrorKind::*;

    str::from_utf8(s.as_bytes())
      .map_err(|e| ParseFromStrError(Utf8Error(e)))
      .and_then(|s| s.parse().map_err(|e| ParseFromStrError(FromStrError(e))))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_byte_char_test() {
    let x: ByteChar = FromByteStr::from_byte_str(ByteStr::from_bytes(b"@")).unwrap();
    assert_eq!(x, ByteChar::new(b'@'));

    let x: Result<ByteChar, _> = FromByteStr::from_byte_str(ByteStr::from_bytes(b""));
    assert!(x.is_err());

    let x: Result<ByteChar, _> = FromByteStr::from_byte_str(ByteStr::from_bytes(b"334"));
    assert!(x.is_err());
  }

  #[test]
  fn parse_byte_string_test() {
    let s = ByteStr::from_bytes(b"Rust 1.15.1");
    let x: ByteString = FromByteStr::from_byte_str(s).unwrap();
    assert_eq!(x, s.to_owned());
  }

  #[test]
  fn parse_from_str_test() {
    let x: usize = FromByteStr::from_byte_str(ByteStr::from_bytes(b"1333")).unwrap();
    assert_eq!(x, 1333);

    let x: f64 = FromByteStr::from_byte_str(ByteStr::from_bytes(b"3.14")).unwrap();
    assert_eq!(x, 3.14);
  }
}
