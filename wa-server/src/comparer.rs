use crate::types::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct SimpleComparer {
    pub ignore_trailing_space: bool,
}

impl Comparer for SimpleComparer {
    fn compare(&self, task: CompareTask, _limit: Option<Limit>) -> WaResult<Comparision> {
        let mut std_reader = BufReader::new(File::open(task.stdout_path)?);
        let mut user_reader = BufReader::new(File::open(task.userout_path)?);

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

            if self.ignore_trailing_space {
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
    use std::fs::File;
    use std::io::Write;
    use Comparision::*;
    use std::path::Path;

    let mut comparer = SimpleComparer {
        ignore_trailing_space: true,
    };

    macro_rules! compare {
        ($ret:expr, $std:expr,$user:expr) => {{
            let mut stdout = File::create("../temp/stdout.out").unwrap();
            let mut user = File::create("../temp/userout.out").unwrap();
            stdout.write_all($std).unwrap();
            user.write_all($user).unwrap();
            drop((stdout, user));
            let ret = comparer.compare(
                CompareTask {
                    working_dir: Path::new("."),
                    stdin_path: "",
                    stdout_path: "../temp/stdout.out",
                    userout_path: "../temp/userout.out",
                },
                Limit::no_effect(),
            );
            assert_eq!(ret.unwrap(), $ret);
        }};
    }

    comparer.ignore_trailing_space = true;

    compare!(AC, b"1 2\n3 4", b"1 2\r\n3 4\n");
    compare!(AC, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    compare!(WA, b"1 2 \n3 4", b"1 2 \r\n3 4 5\n");
    compare!(WA, b"1 2 \n3 4 5", b"1 2 \r\n3 4 \n");
    compare!(WA, b"\n", b"");
    compare!(WA, b"", b"\n");
    compare!(AC, b" \n", b" ");
    compare!(AC, b"1\n", b"1");

    comparer.ignore_trailing_space = false;

    compare!(AC, b"1 2\n3 4", b"1 2\r\n3 4\n");
    compare!(PE, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    compare!(WA, b"1 2 \n3 4", b"1 2 \r\n3 4 5\n");
    compare!(WA, b"1 2 \n3 4 5", b"1 2 \r\n3 4 \n");
    compare!(WA, b"\n", b"");
    compare!(WA, b"", b"\n");
    compare!(AC, b" \n", b" ");
    compare!(AC, b"1\n", b"1");
    compare!(PE, b"1 \n", b"1");
}
