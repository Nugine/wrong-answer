mod component;
mod judge;
mod unit;

pub use self::component::*;
pub use self::judge::*;
pub use self::unit::*;

pub use wa_monitor::Target;
pub use wa_monitor::TargetStatus;

pub enum WaError {
    Io(std::io::Error),
    Compiler(String),
    Monitor(wa_monitor::MonitorErrorKind),
}

pub type WaResult<T> = Result<T, WaError>;
