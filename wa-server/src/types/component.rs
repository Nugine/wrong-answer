use super::Limit;
use super::Target;
use super::TargetStatus;
use super::WaResult;

pub trait SandBox {
    fn run(&self, working_dir: &str, target: &Target, limit: &Limit) -> WaResult<TargetStatus>;
}

pub trait Compiler {
    fn compile(
        &self,
        working_dir: &str,
        source_path: &str,
        binary_path: Option<&str>,
    ) -> WaResult<()>;
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
