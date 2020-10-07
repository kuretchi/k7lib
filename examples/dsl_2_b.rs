// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B

use spella::algebra::structures::Sum;
use spella::sequences::FenwickTree;

use std::io;
use std::iter::FromIterator;

fn main() -> io::Result<()> {
  spella::io::run(None, false, |scanner, writer| {
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

          writeln!(writer, "{}", seq.range_sum(s - 1..t).0)?;
        }
        _ => unreachable!(),
      }
    }

    assert_eq!(
      FenwickTree::from_iter((0..seq.len()).map(|i| seq.point_get(i))),
      seq
    );

    Ok(())
  })
}
