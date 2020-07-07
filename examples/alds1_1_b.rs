// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_B

use spella::num;

fn main() -> std::io::Result<()> {
  spella::io::run(None, false, |scanner, writer| {
    let a = scanner.parse_next::<u32>()?.unwrap();
    let b = scanner.parse_next::<u32>()?.unwrap();
    writeln!(writer, "{}", num::gcd(a, b))
  })
}
