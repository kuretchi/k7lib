// verify-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use k7lib::disjoint_sets::QuickFind;

fn main() -> std::io::Result<()> {
  k7lib::io::run(None, false, |scanner, writer| {
    let n = scanner.parse_next::<usize>()?.unwrap();
    let q = scanner.parse_next::<usize>()?.unwrap();

    let mut uf = QuickFind::new(n);

    for _ in 0..q {
      let t = scanner.parse_next::<u8>()?.unwrap();
      let u = scanner.parse_next::<usize>()?.unwrap();
      let v = scanner.parse_next::<usize>()?.unwrap();

      match t {
        0 => {
          uf.unite(u, v);
        }
        1 => {
          let ans: u8 = uf.belong_to_same_set(u, v).into();
          writeln!(writer, "{}", ans)?;
        }
        _ => unreachable!(),
      }
    }

    Ok(())
  })
}
