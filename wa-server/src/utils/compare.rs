use crate::types::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[deprecated]
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
