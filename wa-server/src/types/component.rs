use super::unit::*;
use super::TargetStatus;
use super::WaResult;

pub struct Target<'a> {
    pub working_dir: &'a str,
    pub bin: &'a str,
    pub args: &'a Vec<&'a str>,
    pub stdin: Option<&'a str>,
    pub stdout: Option<&'a str>,
    pub stderr: Option<&'a str>,
}

pub struct Limit<'a> {
    pub time: MicroSecond,
    pub memory: KiloByte,
    pub output: KiloByte,
    pub security_cfg_path: &'a str,
}

pub trait SandBox {
    fn run(&self, target: Target, limit: Limit) -> WaResult<TargetStatus>;
}

pub struct CompileTask<'a> {
    pub working_dir: &'a str,
    pub source_path: &'a str,
    pub binary_path: &'a str,
    pub ce_message_path: Option<&'a str>,
}

pub trait Compiler {
    fn compile(&self, task: CompileTask, limit: Limit) -> WaResult<TargetStatus>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparision {
    AC,
    PE,
    WA,
}

pub trait Comparer {
    fn compare(&self, std_answer: &str, user_answer: &str) -> WaResult<Comparision>;
}
