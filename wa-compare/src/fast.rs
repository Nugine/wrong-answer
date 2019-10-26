use super::*;

pub fn compare_fast(stdout_path: &Path, userout_path: &Path) -> IoResult<Comparision> {
    let mut std_reader = BufReader::with_capacity(4 * 1024 * 1024, File::open(stdout_path)?);
    let mut user_reader = BufReader::with_capacity(4 * 1024 * 1024, File::open(userout_path)?);

    loop {
        let std_chunk = std_reader.fill_buf()?;
        if std_chunk.is_empty() {
            break;
        }
        let user_chunk = user_reader.fill_buf()?;
        if user_chunk.is_empty() {
            return Ok(Comparision::WA);
        }

        let comp = unsafe { compare_chunk(std_chunk, user_chunk) };

        if !comp {
            return Ok(Comparision::WA);
        }

        let amt = std_chunk.len();
        std_reader.consume(amt);
        let amt = user_chunk.len();
        user_reader.consume(amt);
    }
    {
        if !user_reader.fill_buf()?.is_empty() {
            return Ok(Comparision::WA);
        }
    }

    Ok(Comparision::AC)
}

#[inline(always)]
unsafe fn compare_chunk(std: &[u8], user: &[u8]) -> bool {
    let mut pa = std.as_ptr();
    let mut pb = user.as_ptr();
    let ea = pa.add(std.len()).sub(1);
    let eb = pb.add(user.len()).sub(1);

    while pa < ea && pb < eb {
        if *pa == b'\r' && *pa.add(1) == b'\n' {
            pa = pa.add(1);
        }
        if *pb == b'\r' && *pb.add(1) == b'\n' {
            pb = pb.add(1);
        }
        if *pa != *pb {
            return false;
        }
        pa = pa.add(1);
        pb = pb.add(1);
    }
    if pa < ea && *pa == b'\r' && *pa.add(1) == b'\n' {
        pa = pa.add(1);
    }
    if pb < eb && *pb == b'\r' && *pb.add(1) == b'\n' {
        pb = pb.add(1);
    }
    if pa == ea && pb == eb {
        *pa == *pb
    } else {
        false
    }
}
