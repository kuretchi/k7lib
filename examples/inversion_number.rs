// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_D

use k7lib::string::inversion_number;

fn main() -> std::io::Result<()> {
  k7lib::io::run(None, false, |scanner, writer| {
    let n = scanner.parse_next::<usize>()?.unwrap();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
      a.push(scanner.parse_next::<u32>()?.unwrap());
    }
    let ans: u64 = inversion_number(&mut a);
    writeln!(writer, "{}", ans)
  })
}
