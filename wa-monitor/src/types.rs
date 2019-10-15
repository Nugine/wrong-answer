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

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetStatus {
    pub code: Option<i32>,
    pub signal: Option<i32>,
    pub real_time: u64, // in microseconds
    pub user_time: u64, // in microseconds
    pub sys_time: u64,  // in microseconds
    pub memory: u64,    // in kilobytes
}

use std::fmt::{self, Display};

impl Display for MonitorErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
