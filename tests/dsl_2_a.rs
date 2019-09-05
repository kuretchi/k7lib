#[macro_use]
mod common;

test!("DSL_2_A", |reader, writer| {
  use spella::algebra::{AssociativeMagma, Magma, UnitalMagma};
  use spella::io::Scanner;
  use spella::sequences::SegmentTree;

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

  let mut scanner = Scanner::new(reader);

  macro_rules! scan {
    ($T:ty) => {
      scanner.next::<$T>()?.unwrap()
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
});
