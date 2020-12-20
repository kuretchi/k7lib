// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0516

use spella::algebra::structures::Sum;
use spella::sequences::CumulativeSum;

use std::cmp;
use std::io;
use std::iter::FromIterator;

fn main() -> io::Result<()> {
  spella::io::run(None, false, |scanner, writer| {
    macro_rules! scan {
      ($T:ty) => {
        scanner.parse_next::<$T>()?.unwrap()
      };
    }

    loop {
      let n = scan!(usize);
      let k = scan!(usize);

      if n == 0 && k == 0 {
        break;
      }

      let mut a = Vec::with_capacity(n);

      for _ in 0..n {
        a.push(Sum(scan!(i32)));
      }

      let a = CumulativeSum::from_iter(a);
      let mut acc = Sum(i32::min_value());

      for i in 0..n - (k - 1) {
        acc = cmp::max(acc, a.range_sum(i..i + k));
      }

      writeln!(writer, "{}", acc.0)?;

      assert_eq!(CumulativeSum::from_iter((0..a.len()).map(|i| a.point_get(i))), a);
    }

    Ok(())
  })
}
