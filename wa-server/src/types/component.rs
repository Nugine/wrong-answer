use super::Limit;
use super::Target;
use super::TargetStatus;
use super::WaResult;

pub trait SandBox {
    fn run(&self, working_dir: &str, target: &Target, limit: &Limit) -> WaResult<TargetStatus>;
}

pub struct CompileTask<'a> {
    pub source_path: &'a str,
    pub binary_path: Option<&'a str>,
    pub ce_message_path: Option<&'a str>,
}

pub trait Compiler {
    fn compile(&self, working_dir: &str, task: CompileTask, limit: &Limit) -> WaResult<()>;
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
