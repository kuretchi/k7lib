// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B

use k7lib::algebra::structures::Sum;
use k7lib::sequences::FenwickTree;

use std::io;

fn main() -> io::Result<()> {
  k7lib::io::run(None, false, |scanner, writer| {
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

    assert_eq!((0..seq.len()).map(|i| seq.point_get(i)).collect::<FenwickTree<_>>(), seq);

    Ok(())
  })
}
