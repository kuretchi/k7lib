use crate::byte::{ByteStr, FromByteStr};

use std::io::{self, BufRead};

/// Wraps a reader and tokenize its input.
///
/// Token's separators are `b' '` (0x20) and `b'\n'` (0x0a).
#[derive(Debug)]
pub struct Scanner<R> {
  reader: R,
  buf: Vec<u8>,
  pos: usize,
}

const INITIAL_CAPACITY: usize = 32;

impl<R: BufRead> Scanner<R> {
  /// Creates a new `Scanner`.
  ///
  /// # Examples
  /// From stdin:
  /// ```
  /// # use spella::io::Scanner;
  /// fn main() {
  ///   let stdin = std::io::stdin();
  ///   let mut scanner = Scanner::new(stdin.lock());
  /// }
  /// ```
  pub fn new(reader: R) -> Self {
    Scanner {
      reader: reader,
      buf: Vec::with_capacity(INITIAL_CAPACITY),
      pos: 0,
    }
  }

  /// Parses a next token splitted by whitespaces, and returns it.
  ///
  /// # Examples
  /// ```
  /// # use spella::io::Scanner;
  /// fn main() {
  ///   let mut scanner = Scanner::new(b"3 14" as &[_]);
  ///
  ///   let n: usize = scanner.next().unwrap().expect("parse error");
  ///   assert_eq!(n, 3);
  /// }
  /// ```
  pub fn next<T: FromByteStr>(&mut self) -> io::Result<Result<T, T::Err>> {
    self.next_byte_str().map(T::from_byte_str)
  }

  /// Returns a next token splitted by whitespaces.
  ///
  /// # Examples
  /// ```
  /// # use spella::byte::ByteStr;
  /// # use spella::io::Scanner;
  /// fn main() {
  ///   let mut scanner = Scanner::new(b"Rust 2015" as &[_]);
  ///  
  ///   let s: &ByteStr = scanner.next_byte_str().unwrap();
  ///   assert_eq!(s.as_bytes(), b"Rust" as &[_]);
  /// }
  /// ```
  pub fn next_byte_str(&mut self) -> io::Result<&ByteStr> {
    if self.buf.is_empty() {
      self.read_line()?;
    }

    loop {
      match self.buf.get(self.pos) {
        Some(&b' ') => self.pos += 1,
        Some(&b'\n') => self.read_line()?,
        Some(_) => break,
        None => return Err(io::Error::from(io::ErrorKind::UnexpectedEof)),
      }
    }

    let start = self.pos;
    self.pos += 1;

    loop {
      match self.buf.get(self.pos) {
        Some(&b' ') | Some(&b'\n') | None => break,
        Some(_) => self.pos += 1,
      }
    }

    Ok(ByteStr::from_bytes(&self.buf[start..self.pos]))
  }

  fn read_line(&mut self) -> io::Result<()> {
    self.buf.clear();
    self.pos = 0;
    self.reader.read_until(b'\n', &mut self.buf)?;

    Ok(())
  }
}
