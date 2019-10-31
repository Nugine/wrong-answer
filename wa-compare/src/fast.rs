use super::*;

pub fn compare_fast(stdout_path: &Path, userout_path: &Path) -> IoResult<Comparison> {
    let mut std_reader = BufReader::with_capacity(4 * 1024 * 1024, File::open(stdout_path)?);
    let mut user_reader = BufReader::with_capacity(4 * 1024 * 1024, File::open(userout_path)?);

    loop {
        let std_chunk = std_reader.fill_buf()?;
        if std_chunk.is_empty() {
            break;
        }
        let user_chunk = user_reader.fill_buf()?;
        if user_chunk.is_empty() {
            return Ok(Comparison::WA);
        }

        let comp = unsafe { compare_chunk(std_chunk, user_chunk) };

        if !comp {
            return Ok(Comparison::WA);
        }

        let amt = std_chunk.len();
        std_reader.consume(amt);
        let amt = user_chunk.len();
        user_reader.consume(amt);
    }
    {
        let mut buf = [0_u8; 1];
        if user_reader.read(&mut buf)? != 0 {
            return Ok(Comparison::WA);
        }
    }

    Ok(Comparison::AC)
}

#[inline(always)]
unsafe fn compare_chunk(std: &[u8], user: &[u8]) -> bool {
    let std_len = std.len();
    let user_len: usize = user.len();

    if std_len == 0 || user_len == 0 {
        return std_len == user_len;
    }

    let mut pa = std.as_ptr();
    let mut pb = user.as_ptr();
    let ea = pa.add(std_len - 1);
    let eb = pb.add(user_len - 1);

    loop {
        if *pa != *pb {
            let ta = pa < ea && *pa == b'\r' && *pa.add(1) == b'\n';
            if ta {
                pa = pa.add(1);
            }
            let tb = pb < eb && *pb == b'\r' && *pb.add(1) == b'\n';
            if tb {
                pb = pb.add(1);
            }
            if ta | tb {
                continue;
            }
            return false;
        }
        pa = pa.add(1);
        pb = pb.add(1);

        let ba = pa > ea;
        let bb = pb > eb;
        if ba | bb {
            return ba & bb;
        }
    }
}
