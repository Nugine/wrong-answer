mod ascii;
mod fast;
mod utf8;

use std::fs::File;
use std::io::Read;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    AC,
    WA,
    PE,
}

pub use ascii::compare_ascii;
pub use fast::compare_fast;
pub use utf8::compare_utf8;

#[test]
fn test_compare() {
    test_compare_utf8();
    test_compare_ascii();
    test_compare_fast();
}

#[cfg(test)]
macro_rules! judge {
    (@fast $ret:expr, $std:expr,$user:expr)=>{{
        let stdout_path = "../temp/stdout.out";
        let userout_path = "../temp/userout.out";
        std::fs::write(stdout_path, $std).unwrap();
        std::fs::write(userout_path, $user).unwrap();
        let ret = compare_fast(
            Path::new(stdout_path),
            Path::new(userout_path),
        );
        assert_eq!(ret.unwrap(), $ret);
    }};
    (@judge $func:expr,$allow: expr, $ret:expr, $std:expr,$user:expr) => {{
        let stdout_path = "../temp/stdout.out";
        let userout_path = "../temp/userout.out";
        std::fs::write(stdout_path, $std).unwrap();
        std::fs::write(userout_path, $user).unwrap();
        let ret = $func(
            $allow,
            Path::new(stdout_path),
            Path::new(userout_path),
        );
        assert_eq!(ret.unwrap(), $ret);
    }};

    (@strict @utf8 $ret:expr, $std:expr,$user:expr) => {
        judge!(@judge compare_utf8, false, $ret, $std, $user);
    };
    (@permissive @utf8 $ret:expr, $std:expr,$user:expr) => {
        judge!(@judge compare_utf8, true,$ret, $std, $user);
    };
    (@strict @ascii $ret:expr, $std:expr,$user:expr) => {
        judge!(@judge compare_ascii, false, $ret, $std, $user);
    };
    (@permissive @ascii $ret:expr, $std:expr,$user:expr) => {
        judge!(@judge compare_ascii, true,$ret, $std, $user);
    };

}

#[cfg(test)]
fn test_compare_utf8() {
    use Comparison::*;

    judge!(@permissive @utf8  AC, b"1 2\n3 4", b"1 2\r\n3 4\n");
    judge!(@permissive @utf8  AC, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    judge!(@permissive @utf8  WA, b"1 2 \n3 4", b"1 2 \r\n3 4 5\n");
    judge!(@permissive @utf8  WA, b"1 2 \n3 4 5", b"1 2 \r\n3 4 \n");
    judge!(@permissive @utf8  WA, b"\n", b"");
    judge!(@permissive @utf8  WA, b"", b"\n");
    judge!(@permissive @utf8  AC, b" \n", b" ");
    judge!(@permissive @utf8  AC, b"1\n", b"1");

    judge!(@strict @utf8 AC, b"1 2\n3 4", b"1 2\r\n3 4\n");
    judge!(@strict @utf8 PE, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    judge!(@strict @utf8 WA, b"1 2 \n3 4", b"1 2 \r\n3 4 5\n");
    judge!(@strict @utf8 WA, b"1 2 \n3 4 5", b"1 2 \r\n3 4 \n");
    judge!(@strict @utf8 WA, b"\n", b"");
    judge!(@strict @utf8 WA, b"", b"\n");
    judge!(@strict @utf8 AC, b" \n", b" ");
    judge!(@strict @utf8 AC, b"1\n", b"1");
    judge!(@strict @utf8 PE, b"1 \n", b"1");
}

#[cfg(test)]
fn test_compare_ascii() {
    use Comparison::*;

    judge!(@permissive @ascii  AC, b"1 2\n3 4", b"1 2\r\n3 4\n");
    judge!(@permissive @ascii  AC, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    judge!(@permissive @ascii  WA, b"1 2 \n3 4", b"1 2 \r\n3 4 5\n");
    judge!(@permissive @ascii  WA, b"1 2 \n3 4 5", b"1 2 \r\n3 4 \n");
    judge!(@permissive @ascii  WA, b"\n", b"");
    judge!(@permissive @ascii  WA, b"", b"\n");
    judge!(@permissive @ascii  AC, b" \n", b" ");
    judge!(@permissive @ascii  AC, b"1\n", b"1");

    judge!(@strict @ascii AC, b"1 2\n3 4", b"1 2\r\n3 4\n");
    judge!(@strict @ascii PE, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    judge!(@strict @ascii WA, b"1 2 \n3 4", b"1 2 \r\n3 4 5\n");
    judge!(@strict @ascii WA, b"1 2 \n3 4 5", b"1 2 \r\n3 4 \n");
    judge!(@strict @ascii WA, b"\n", b"");
    judge!(@strict @ascii WA, b"", b"\n");
    judge!(@strict @ascii AC, b" \n", b" ");
    judge!(@strict @ascii AC, b"1\n", b"1");
    judge!(@strict @ascii PE, b"1 \n", b"1");
}

#[cfg(test)]
fn test_compare_fast() {
    use Comparison::*;

    judge!(@fast WA, b"1 2\n3 4", b"1 2\r\n3 4\n");
    judge!(@fast WA, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    judge!(@fast WA, b"1 2 \n3 4", b"1 2 \r\n3 4 5\n");
    judge!(@fast WA, b"1 2 \n3 4 5", b"1 2 \r\n3 4 \n");
    judge!(@fast WA, b"\n", b"");
    judge!(@fast WA, b"", b"\n");
    judge!(@fast WA, b" \n", b" ");
    judge!(@fast WA, b"1\n", b"1");
    judge!(@fast WA, b"1 \n", b"1");
    judge!(@fast WA, b"1\r\n", b"1\r");

    judge!(@fast AC, b"1 2\n3 4", b"1 2\r\n3 4");
    judge!(@fast AC, b"1 2 \n3 4", b"1 2 \r\n3 4");
    judge!(@fast AC, b"\n", b"\r\n");
    judge!(@fast AC, b"", b"");
    judge!(@fast AC, b" ", b" ");
    judge!(@fast AC, b"1\n", b"1\n");
    judge!(@fast AC, b"1\n\r", b"1\n\r");
    judge!(@fast AC, b"\r\n", b"\r\n");
    judge!(@fast AC, b"\r\n\r\n", b"\r\n\r\n");
    judge!(@fast AC, b"\r\n\n", b"\r\n\r\n");
}
