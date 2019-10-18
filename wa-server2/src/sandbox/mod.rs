mod bare_monitor;
pub use bare_monitor::BareMonitorSandBox;

use crate::types::*;

pub struct Target<'a> {
    pub working_dir: &'a Path,
    pub bin: &'a str,
    pub args: &'a Vec<&'a str>,
    pub stdin: Option<&'a Path>,
    pub stdout: Option<&'a Path>,
    pub stderr: Option<&'a Path>,
}

pub struct Limit<'a> {
    pub time: MilliSecond,
    pub memory: KiloByte,
    pub output: KiloByte,
    pub security_cfg_path: Option<&'a str>,
}

pub trait SandBox {
    fn run(&self, target: Target, limit: Option<Limit>) -> WaResult<TargetStatus>;
}

impl Limit<'static> {
    pub fn no_effect() -> Option<Self> {
        None
    }
}
