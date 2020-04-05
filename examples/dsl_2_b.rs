// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B

use spella::algebra::structures::{
  AssociativeMagma, CommutativeMagma, InvertibleMagma, Magma, UnitalMagma,
};
use spella::io::Scanner;
use spella::sequences::FenwickTree;

use std::io::{self, prelude::*};
use std::iter::FromIterator;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sum(pub i32);

impl Magma for Sum {
  fn op(&self, rhs: &Self) -> Self {
    Sum(self.0 + rhs.0)
  }
}

impl AssociativeMagma for Sum {}

impl CommutativeMagma for Sum {}

impl UnitalMagma for Sum {
  fn identity() -> Self {
    Sum(0)
  }
}

impl InvertibleMagma for Sum {
  fn invert(&self) -> Self {
    Sum(-self.0)
  }
}

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let reader = stdin.lock();
  let stdout = io::stdout();
  let mut writer = std::io::BufWriter::new(stdout);
  let mut scanner = Scanner::new(reader);

  macro_rules! scan {
    ($T:ty) => {
      scanner.parse_next::<$T>()?.unwrap()
    };
  }

  let n = scan!(usize);
  let q = scan!(usize);

  let mut seq = FenwickTree::new(n);

  for _ in 0..q {
    let com = scan!(usize);

    match com {
      0 => {
        let i = scan!(usize);
        let x = scan!(i32);

        seq.point_append(i - 1, &Sum(x));
      }
      1 => {
        let s = scan!(usize);
        let t = scan!(usize);

        writeln!(writer, "{}", seq.fold(s - 1..t).0)?;
      }
      _ => unreachable!(),
    }
  }

  assert_eq!(
    FenwickTree::from_iter((0..seq.len()).map(|i| seq.get(i))),
    seq
  );

  Ok(())
}
