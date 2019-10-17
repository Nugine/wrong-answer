mod component;
mod judge;
mod unit;

pub use self::component::*;
pub use self::judge::*;
pub use self::unit::*;

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
    Compiler(String),
    Monitor(#[from] MonitorErrorKind),
}

pub type WaResult<T> = Result<T, WaError>;

impl Display for WaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
