use crate::types::*;

pub struct CompileTask<'a> {
    pub workspace: &'a Path,
    pub src_path: &'a Path,
    pub compile_message_path: &'a Path,
}

pub struct CaseTask<'a> {
    workspace: PathBuf,
    submission: &'a Submission,
    src_path: PathBuf,
    bin_path: Option<PathBuf>,
    stdin_path: PathBuf,
    stdout_path: PathBuf,
    userout_path: PathBuf,
    act_path: Option<PathBuf>,
    spj_path: Option<PathBuf>,
}

pub enum CompileResult {
    Success(PathBuf),
    Failure(String),
}

pub trait LanguageBroker {
    fn save_source(&self, submission: &Submission, workspace: &Path) -> WaResult<PathBuf>;
    fn compile(&self, task: CompileTask) -> WaResult<CompileResult>;
    fn run_case(&self, task: &CaseTask) -> WaResult<JudgeCaseResult>;
}
