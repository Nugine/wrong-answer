use super::*;

pub fn compare_ascii(
    allow_pe: bool,
    stdout_path: &Path,
    userout_path: &Path,
) -> IoResult<Comparision> {
    let mut std_reader = BufReader::new(File::open(stdout_path)?);
    let mut user_reader = BufReader::new(File::open(userout_path)?);

    let mut std_buf = <Vec<u8>>::with_capacity(64);
    let mut user_buf = <Vec<u8>>::with_capacity(64);

    while let Some(std_line) = next_line(&mut std_buf, &mut std_reader)? {
        let user_line = match next_line(&mut user_buf, &mut user_reader)? {
            Some(l) => l,
            None => return Ok(Comparision::WA),
        };

        let std_parts = trim(std_line).split(u8::is_ascii_whitespace);
        let mut user_parts = trim(user_line).split(u8::is_ascii_whitespace);

        for std_part in std_parts {
            let user_part = match user_parts.next() {
                None => return Ok(Comparision::WA),
                Some(p) => p,
            };
            if std_part != user_part {
                return Ok(Comparision::WA);
            }
        }
        if let Some(user_part) = user_parts.next() {
            if !user_part.is_empty() {
                return Ok(Comparision::WA);
            }
        }
        if !allow_pe && std_line != user_line {
            return Ok(Comparision::PE);
        }
    }
    {
        if next_line(&mut user_buf, &mut user_reader)?.is_some() {
            return Ok(Comparision::WA);
        }
    }

    Ok(Comparision::AC)
}

fn next_line<'a>(
    buf: &'a mut Vec<u8>,
    reader: &mut BufReader<File>,
) -> IoResult<Option<&'a Vec<u8>>> {
    buf.clear();
    if reader.read_until(b'\n', buf)? == 0 {
        return Ok(None);
    }
    if let Some(b'\n') = buf.last() {
        buf.pop();
        if let Some(b'\r') = buf.last() {
            buf.pop();
        }
    }
    Ok(Some(buf))
}

fn trim(buf: &[u8]) -> &[u8] {
    if buf.is_empty() {
        return buf;
    }

    let mut s = 0;
    let mut e = buf.len();
    while s < e {
        let &byte = unsafe { buf.get_unchecked(s) };
        if byte.is_ascii_whitespace() {
            s += 1;
        } else {
            break;
        }
    }
    while s < e {
        let &byte = unsafe { buf.get_unchecked(e - 1) };
        if byte.is_ascii_whitespace() {
            e -= 1;
        } else {
            break;
        }
    }
    &buf[s..e]
}
