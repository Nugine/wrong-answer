use std::ffi::CString;
use std::path::Path;
use wa_monitor::types::MonitorErrorKind;

pub fn mkfifo(path: &Path) -> Result<(), MonitorErrorKind> {
    let path = path.to_str().unwrap();
    let path: CString = CString::new(path).unwrap();
    unsafe {
        let ret = libc::mkfifo(path.as_ptr(), 0o666);
        if ret < 0 {
            log::error!("mkfifo error: errno = {}", get_errno());
            Err(MonitorErrorKind::FifoError)
        } else {
            Ok(())
        }
    }
}

#[inline(always)]
unsafe fn get_errno() -> libc::c_int {
    *libc::__errno_location()
}
