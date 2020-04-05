// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A

use spella::algebra::structures::{AssociativeMagma, Magma, UnitalMagma};
use spella::io::Scanner;
use spella::sequences::SegmentTree;

use std::io::{self, prelude::*};
use std::iter::FromIterator;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Min(pub i32);

impl Magma for Min {
  fn op(&self, rhs: &Self) -> Self {
    *if self < rhs { self } else { rhs }
  }
}

impl AssociativeMagma for Min {}

impl UnitalMagma for Min {
  fn identity() -> Self {
    Min(2147483647)
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

  let mut seq = SegmentTree::new(n);

  for _ in 0..q {
    let com = scan!(usize);

    match com {
      0 => {
        let i = scan!(usize);
        let x = scan!(i32);

        *seq.get_mut(i) = Min(x);
      }
      1 => {
        let s = scan!(usize);
        let t = scan!(usize);

        writeln!(writer, "{}", seq.fold(s..t + 1).0)?;
      }
      _ => unreachable!(),
    }
  }

  assert_eq!(
    SegmentTree::from_iter((0..seq.len()).map(|i| seq.get(i)).cloned()),
    seq
  );

  Ok(())
}
