use libc::c_char;
use libc::c_int;
use serde::Serialize;
use std::ffi::CString;

macro_rules! check_os_error {
    ($ret: expr, $kind: expr) => {{
        if $ret < 0 {
            let errno = get_errno();
            log::error!("kind = {:?}, errno = {:?}", $kind, errno);
            std::process::exit($kind as i32)
        }
    }};
    (@errno $errno: expr, $kind: expr) => {{
        if $errno != 0 {
            log::error!("kind = {:?}, errno = {:?}", $kind, $errno);
            std::process::exit($kind as i32)
        }
    }};
    (@ret $ret:expr) => {{
        if $ret < 0 {
            let errno = get_errno();
            return errno;
        }
    }};
}

#[inline(always)]
unsafe fn get_errno() -> c_int {
    *libc::__errno_location()
}

#[derive(Debug)]
pub enum MonitorErrorKind {
    PipeError = 2,
    ForkError = 3,
    PipeReadError = 4,
    Wait4Error = 5,
    ChildError = 6,
    ExecvpError = 7,
}

pub struct Target {
    pub bin: CString,
    pub args: Vec<CString>,
    pub stdin: Option<CString>,
    pub stdout: Option<CString>,
    pub stderr: Option<CString>,
}

#[derive(Debug, Serialize)]
pub struct TargetStatus {
    pub code: Option<i32>,
    pub signal: Option<i32>,
    pub real_time: u64, // in microseconds
    pub user_time: u64, // in microseconds
    pub sys_time: u64,  // in microseconds
    pub memory: u64,    // in kilobytes
}

impl Target {
    pub fn run(&self) -> TargetStatus {
        unsafe {
            let pid = self.spawn();
            Target::wait(pid)
        }
    }
}

impl Target {
    unsafe fn spawn(&self) -> libc::pid_t {
        let argv = self.gen_argv();

        let (rx_fd, tx_fd) = {
            let mut fds: [c_int; 2] = [0; 2];
            let ret = libc::pipe(fds.as_mut_ptr());
            check_os_error!(ret, MonitorErrorKind::PipeError);
            (fds[0], fds[1])
        };

        let ret = libc::fork();
        check_os_error!(ret, MonitorErrorKind::ForkError);
        let pid = ret;
        if pid == 0 {
            // child process begin
            let _ = libc::close(rx_fd);

            let errno = self.dup_std();
            // send errno
            let bytes = (errno as i32).to_ne_bytes();
            let _ = libc::write(tx_fd, bytes.as_ptr() as *const libc::c_void, bytes.len());
            let _ = libc::close(tx_fd);

            if errno != 0 {
                libc::exit(errno);
            }

            libc::execvp(argv[0], argv.as_ptr());
            // child process end

            let errno = get_errno();
            check_os_error!(@errno errno,MonitorErrorKind::ExecvpError);
            std::process::exit(0);
            // child process fail
        }
        std::mem::forget(argv);

        {
            let _ = libc::close(tx_fd);
            // receive errno
            let mut bytes: [u8; 4] = [0; 4];
            let ret = libc::read(rx_fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len());
            let _ = libc::close(rx_fd);

            if ret < 0 {
                let _ = libc::kill(pid, libc::SIGKILL);
            }
            check_os_error!(ret, MonitorErrorKind::PipeReadError);

            let errno = i32::from_ne_bytes(bytes);
            check_os_error!(@errno errno,MonitorErrorKind::ChildError);
        }

        pid
    }

    unsafe fn wait(pid: libc::pid_t) -> TargetStatus {
        let mut status = std::mem::zeroed::<c_int>();
        let mut ru = std::mem::zeroed::<libc::rusage>();
        let t0 = std::time::Instant::now();

        {
            let ret = libc::wait4(
                pid,
                &mut status as *mut c_int,
                libc::WSTOPPED,
                &mut ru as *mut libc::rusage,
            );
            if ret < 0 {
                let _ = libc::kill(pid, libc::SIGKILL);
            }
            check_os_error!(ret, MonitorErrorKind::Wait4Error);
        }

        let real_time = t0.elapsed().as_micros() as u64;
        let (code, signal) = {
            let exited = libc::WIFEXITED(status);
            if exited {
                (Some(libc::WEXITSTATUS(status)), None)
            } else {
                (None, Some(libc::WTERMSIG(status)))
            }
        };

        let user_time = (ru.ru_utime.tv_sec as u64 * 1000_000) + (ru.ru_utime.tv_usec as u64);
        let sys_time = (ru.ru_stime.tv_sec as u64 * 1000_000) + (ru.ru_stime.tv_usec as u64);
        let memory = ru.ru_maxrss as u64;

        TargetStatus {
            code,
            signal,
            real_time,
            user_time,
            sys_time,
            memory,
        }
    }
}

impl Target {
    unsafe fn dup_std(&self) -> libc::c_int {
        if let Some(ref input_path) = self.stdin {
            let input_fd = open_read_fd(input_path.as_ptr());
            check_os_error!(@ret input_fd);
            let stdin_fd = libc::STDIN_FILENO;
            let ret = libc::dup2(input_fd, stdin_fd);
            check_os_error!(@ret ret);
        }
        if let Some(ref output_path) = self.stdout {
            let output_fd = open_write_fd(output_path.as_ptr());
            check_os_error!(@ret output_fd);
            let stdout_fd = libc::STDOUT_FILENO;
            let ret = libc::dup2(output_fd, stdout_fd);
            check_os_error!(@ret ret);
        }
        if let Some(ref error_path) = self.stderr {
            let error_fd = open_write_fd(error_path.as_ptr());
            check_os_error!(@ret error_fd);
            let stderr_fd = libc::STDERR_FILENO;
            let ret = libc::dup2(error_fd, stderr_fd);
            check_os_error!(@ret ret);
        }
        0
    }

    unsafe fn gen_argv(&self) -> Vec<*const c_char> {
        let mut argv: Vec<*const c_char> = Vec::with_capacity(self.args.len() + 2);
        argv.push(self.bin.as_ptr());
        argv.extend(self.args.iter().map(|s| s.as_ptr()));
        argv.push(libc::PT_NULL as *const c_char);
        argv
    }
}

unsafe fn open_read_fd(path: *const c_char) -> c_int {
    use libc::{AT_FDCWD, O_RDONLY};
    libc::openat(AT_FDCWD, path, O_RDONLY, 0o666)
}

unsafe fn open_write_fd(path: *const c_char) -> c_int {
    use libc::{AT_FDCWD, O_CREAT, O_TRUNC, O_WRONLY};
    libc::openat(AT_FDCWD, path, O_WRONLY | O_CREAT | O_TRUNC, 0o666)
}
