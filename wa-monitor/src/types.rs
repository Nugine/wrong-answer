use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use thiserror::Error;

#[derive(Debug, Error, FromPrimitive, ToPrimitive)]
pub enum MonitorErrorKind {
    Unknown = 1, // outside
    PipeError = 2,
    ForkError = 3,
    PipeReadError = 4,
    Wait4Error = 5,
    ChildError = 6,
    FifoError = 7,
    ThreadError = 8,

    // can not distinguish ExecvpError from user runtime error, use special number here
    ExecvpError = 42,
}

pub struct Target {
    pub bin: CString,
    pub args: Vec<CString>,
    pub stdin: Option<CString>,
    pub stdout: Option<CString>,
    pub stderr: Option<CString>,
}

type MilliSecond = u64;
type KiloByte = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetStatus {
    pub code: Option<i32>,
    pub signal: Option<i32>,
    pub real_time: MilliSecond,
    pub user_time: MilliSecond,
    pub sys_time: MilliSecond,
    pub memory: KiloByte,
}

use std::fmt::{self, Display};

impl Display for MonitorErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
