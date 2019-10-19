use crate::types::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn compare(
    ignore_trailing_space: bool,
    stdout_path: &Path,
    userout_path: &Path,
) -> WaResult<Comparision> {
    let mut std_reader = BufReader::new(File::open(stdout_path)?);
    let mut user_reader = BufReader::new(File::open(userout_path)?);

    let mut std_line = String::new();
    let mut user_line = String::new();

    loop {
        std_line.clear();
        let len = std_reader.read_line(&mut std_line)?;
        if len == 0 {
            break;
        }

        user_line.clear();
        let len = user_reader.read_line(&mut user_line)?;
        if len == 0 {
            return Ok(Comparision::WA);
        }

        let std_line = trim_endline(&std_line);
        let user_line = trim_endline(&user_line);

        let std_line_trimed = std_line.trim_end();
        let user_line_trimed = user_line.trim_end();
        let std_line_tail = &std_line[std_line_trimed.len()..];
        let user_line_tail = &user_line[user_line_trimed.len()..];

        if std_line_trimed != user_line_trimed {
            return Ok(Comparision::WA);
        }

        if ignore_trailing_space {
            continue;
        }

        if std_line_tail != user_line_tail {
            return Ok(Comparision::PE);
        }
    }
    {
        let len = user_reader.read_line(&mut user_line)?;
        if len > 0 {
            return Ok(Comparision::WA);
        }
    }

    Ok(Comparision::AC)
}

fn trim_endline(s: &str) -> &str {
    let bytes = s.as_bytes();
    let bytes_len = bytes.len();

    if let Some(b'\n') = bytes.get(bytes_len - 1) {
        if let Some(b'\r') = bytes.get(bytes_len - 2) {
            &s[..bytes_len - 2]
        } else {
            &s[..bytes_len - 1]
        }
    } else {
        &s[..]
    }
}

#[test]
fn test_simple_comparer() {
    use std::path::Path;
    use Comparision::*;

    macro_rules! judge {
        (@judge $ignore:expr, $ret:expr, $std:expr,$user:expr) => {{
            let stdout_path = "../temp/stdout.out";
            let userout_path = "../temp/userout.out";
            std::fs::write(stdout_path, $std).unwrap();
            std::fs::write(userout_path, $user).unwrap();
            let ret = compare(
                $ignore,
                Path::new(stdout_path),
                Path::new(userout_path),
            );
            assert_eq!(ret.unwrap(), $ret);
        }};
        (@strict $ret:expr, $std:expr,$user:expr) => {
            judge!(@judge false, $ret, $std, $user);
        };
        (@ignore $ret:expr, $std:expr,$user:expr) => {
            judge!(@judge true, $ret, $std, $user);
        };
    }

    judge!(@ignore AC, b"1 2\n3 4", b"1 2\r\n3 4\n");
    judge!(@ignore AC, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    judge!(@ignore WA, b"1 2 \n3 4", b"1 2 \r\n3 4 5\n");
    judge!(@ignore WA, b"1 2 \n3 4 5", b"1 2 \r\n3 4 \n");
    judge!(@ignore WA, b"\n", b"");
    judge!(@ignore WA, b"", b"\n");
    judge!(@ignore AC, b" \n", b" ");
    judge!(@ignore AC, b"1\n", b"1");

    judge!(@strict AC, b"1 2\n3 4", b"1 2\r\n3 4\n");
    judge!(@strict PE, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    judge!(@strict WA, b"1 2 \n3 4", b"1 2 \r\n3 4 5\n");
    judge!(@strict WA, b"1 2 \n3 4 5", b"1 2 \r\n3 4 \n");
    judge!(@strict WA, b"\n", b"");
    judge!(@strict WA, b"", b"\n");
    judge!(@strict AC, b" \n", b" ");
    judge!(@strict AC, b"1\n", b"1");
    judge!(@strict PE, b"1 \n", b"1");
}