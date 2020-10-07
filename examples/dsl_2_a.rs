// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A

use spella::algebra::structures::Min;
use spella::sequences::SegmentTree;

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

    let mut seq = SegmentTree::new(n);

    for _ in 0..q {
      let com = scan!(usize);

      match com {
        0 => {
          let i = scan!(usize);
          let x = scan!(i32);

          *seq.point_get_mut(i) = Min(x);
        }
        1 => {
          let s = scan!(usize);
          let t = scan!(usize);

          writeln!(writer, "{}", seq.range_sum(s..t + 1).0)?;
        }
        _ => unreachable!(),
      }
    }

    assert_eq!(
      SegmentTree::from_iter((0..seq.len()).map(|i| seq.point_get(i)).cloned()),
      seq
    );

    Ok(())
  })
}
