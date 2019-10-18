use super::unit::*;
use super::TargetStatus;
use super::WaResult;
use std::path::Path;

pub struct Target<'a> {
    pub working_dir: &'a Path,
    pub bin: &'a str,
    pub args: &'a Vec<&'a str>,
    pub stdin: Option<&'a str>,
    pub stdout: Option<&'a str>,
    pub stderr: Option<&'a str>,
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

pub struct CompileTask<'a> {
    pub working_dir: &'a Path,
    pub source_path: &'a str,
    pub binary_path: &'a str,
    pub ce_message_path: &'a str,
}

pub trait Compiler {
    fn compile(&self, task: CompileTask, limit: Option<Limit>) -> WaResult<TargetStatus>;
}

pub struct CompareTask<'a> {
    pub working_dir: &'a Path,
    pub stdin_path: &'a str,
    pub stdout_path: &'a str,
    pub userout_path: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparision {
    AC = 0,
    WA = 1,
    PE = 2,
}

pub trait Comparer {
    fn compare(&self, task: CompareTask, limit: Option<Limit>) -> WaResult<Comparision>;
}

impl Limit<'static> {
    pub const fn no_effect() -> Option<Self> {
        None
    }
}
