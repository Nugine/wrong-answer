use crate::types::*;

pub struct CompileTask<'a> {
    pub working_dir: &'a Path,
    pub src_path: &'a Path,
    pub compile_message_path: &'a Path,
}

pub struct CaseTask<'a> {
    pub working_dir: &'a Path,
    pub submission: &'a Submission,
    pub src_path: PathBuf,
    pub bin_path: Option<PathBuf>,
    pub stdin_path: PathBuf,
    pub stdout_path: PathBuf,
    pub userout_path: PathBuf,
    pub act_path: Option<PathBuf>,
    pub spj_path: Option<PathBuf>,
}

pub enum CompileResult {
    Success(PathBuf),
    CE(String),
    CLE,
    None,
}

pub trait LanguageBroker {
    fn save_source(&self, submission: &Submission, workspace: &Path) -> WaResult<PathBuf>;
    fn compile(&self, task: CompileTask) -> WaResult<CompileResult>;
    fn run_case(&self, task: &CaseTask) -> WaResult<JudgeCaseResult>;
}

impl Language {
    pub fn get_broker(&self) -> Box<dyn LanguageBroker> {
        unimplemented!()
    }
}
