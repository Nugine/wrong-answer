mod unit {
    pub type KiloByte = u64;
    pub type MegaByte = u64;
    pub type Second = u64;
    pub type MilliSecond = u64;
}
mod judge;
mod lang;
mod sandbox;

pub use judge::*;
pub use lang::*;
pub use sandbox::*;
pub use unit::*;

pub use num_traits::FromPrimitive;
pub use serde::{Deserialize, Serialize};
pub use std::collections::HashMap;
pub use std::path::{Path, PathBuf};
pub use std::sync::{Arc, RwLock};
pub use wa_monitor::types::{MonitorErrorKind, TargetStatus};

pub type DataLock = RwLock<HashMap<u64, u64>>;

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
    R2d2(
        #[from]
        #[source]
        r2d2::Error,
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
