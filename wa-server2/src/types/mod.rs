mod unit {
    pub type KiloByte = u64;
    pub type MegaByte = u64;
    pub type Second = u64;
    pub type MilliSecond = u64;
}
mod judge;

pub use judge::*;
pub use num_traits::FromPrimitive;
pub use std::path::{Path, PathBuf};
pub use unit::*;
pub use wa_monitor::types::{MonitorErrorKind, TargetStatus};

use std::fmt::{self, Display};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WaError {
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),
    Redis(
        #[from]
        #[source]
        redis::RedisError,
    ),
    Channel(&'static str),
    Monitor(#[from] MonitorErrorKind),
}

pub type WaResult<T> = Result<T, WaError>;

impl Display for WaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
