use crate::io::Scanner;

use std::convert;
use std::io::{self, prelude::*, BufWriter, StdinLock};
use std::thread;

pub fn run<F>(stack_size: Option<usize>, interactive: bool, f: F) -> io::Result<()>
where
  F: FnOnce(&mut Scanner<StdinLock>, &mut dyn Write) -> io::Result<()>,
  F: Send + 'static,
{
  let f = move || {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut scanner = Scanner::new(reader);
    let stdout = io::stdout();
    let writer = stdout.lock();
    macro_rules! with {
      ($wrapper:expr) => {{
        let mut writer = $wrapper(writer);
        f(&mut scanner, &mut writer)?;
        writer.flush()
      }};
    }
    if cfg!(debug_assertions) || interactive {
      with!(convert::identity)
    } else {
      with!(BufWriter::new)
    }
  };

  match stack_size {
    Some(stack_size) => thread::Builder::new().stack_size(stack_size).spawn(f)?.join().unwrap(),
    None => f(),
  }
}
