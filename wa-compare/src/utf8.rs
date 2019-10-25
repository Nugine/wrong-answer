use super::*;

pub fn compare_utf8(
    allow_pe: bool,
    stdout_path: &Path,
    userout_path: &Path,
) -> IoResult<Comparision> {
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

        let std_parts = std_line.split_whitespace();
        let mut user_parts = user_line.split_whitespace();
        for std_part in std_parts {
            let user_part = match user_parts.next() {
                None => return Ok(Comparision::WA),
                Some(p) => p,
            };
            if std_part != user_part {
                return Ok(Comparision::WA);
            }
        }
        if user_parts.next().is_some() {
            return Ok(Comparision::WA);
        }
        if !allow_pe && std_line != user_line {
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
