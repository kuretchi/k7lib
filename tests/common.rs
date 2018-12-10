use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str;
use std::thread;
use std::time::Duration;

#[macro_export]
macro_rules! test {
  ($problem_id:expr, |$reader:ident, $writer:ident| $body:expr) => {
    extern crate spella;

    use std::io::Write;

    #[test]
    fn test() {
      common::test($problem_id, |$reader, $writer| {
        $body;
        Ok(())
      })
      .unwrap();
    }
  };
}

pub fn test<F>(problem_id: &str, test_single_case: F) -> io::Result<()>
where
  F: Fn(BufReader<File>, &mut Vec<u8>) -> io::Result<()>,
{
  let mut stderr = io::stderr();

  for test_case in (1..).map(|serial| download_test_case(problem_id, serial)) {
    let mut test_case = if let Some(test_case) = test_case? {
      test_case
    } else {
      break;
    };

    let mut output = vec![];
    test_single_case(BufReader::new(test_case.input_file), &mut output)?;

    let mut expected_output = vec![];
    test_case.output_file.read_to_end(&mut expected_output)?;

    if output == expected_output {
      writeln!(stderr, "case {} ... ok", test_case.serial)?;
    } else {
      panic!(
        "case {} ... wrong answer:\n[expected]\n{}\n[actual]\n{}",
        test_case.serial,
        str::from_utf8(&expected_output).unwrap(),
        String::from_utf8_lossy(&output),
      );
    }
  }

  Ok(())
}

struct TestCase {
  serial: usize,
  input_file: File,
  output_file: File,
}

fn download_test_case(problem_id: &str, serial: usize) -> io::Result<Option<TestCase>> {
  let uri = format!(
    "https://judgedat.u-aizu.ac.jp/testcases/{}/{}",
    problem_id, serial
  );

  let dir_path = PathBuf::from(format!("tests/testcases/{}", problem_id));
  fs::create_dir_all(&dir_path)?;

  let input_file_path = dir_path.join(format!("{}.in", serial));

  if !input_file_path.exists() {
    {
      let status_code = {
        let output = curl(&uri)
          .args(&["-w", "%{http_code}", "-o", "/dev/null"])
          .stderr(Stdio::inherit())
          .output()?;

        assert!(output.status.success());

        String::from_utf8(output.stdout)
          .unwrap()
          .parse::<u16>()
          .unwrap()
      };

      if status_code != 200 {
        return Ok(None);
      }
    }

    let input_file = File::create(&input_file_path)?;

    let status = curl(&format!("{}/in", uri))
      .stdout(unsafe { Stdio::from_raw_fd(input_file.into_raw_fd()) })
      .status()?;

    assert!(status.success());
  }

  let output_file_path = dir_path.join(format!("{}.out", serial));

  if !output_file_path.exists() {
    let output_file = File::create(&output_file_path)?;

    let status = curl(&format!("{}/out", uri))
      .stdout(unsafe { Stdio::from_raw_fd(output_file.into_raw_fd()) })
      .status()?;

    assert!(status.success());
  }

  Ok(Some(TestCase {
    serial: serial,
    input_file: File::open(input_file_path)?,
    output_file: File::open(output_file_path)?,
  }))
}

fn curl(uri: &str) -> Command {
  thread::sleep(Duration::from_secs(1));

  let mut command = Command::new("curl");
  command.arg(uri);
  command
}
