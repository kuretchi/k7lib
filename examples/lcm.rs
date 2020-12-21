// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_C

use k7lib::num;

fn main() -> std::io::Result<()> {
  k7lib::io::run(None, false, |scanner, writer| {
    let n = scanner.parse_next::<usize>()?.unwrap();
    let mut lcm = 1;
    for _ in 0..n {
      let a = scanner.parse_next::<u32>()?.unwrap();
      lcm = num::lcm(lcm, a).unwrap();
    }
    writeln!(writer, "{}", lcm)
  })
}
