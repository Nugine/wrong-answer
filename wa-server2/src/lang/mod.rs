mod gcc;
pub use gcc::Gcc;
mod helper;

use crate::types::*;

pub struct CompileTask<'a> {
    pub working_dir: &'a Path,
    pub src_path: &'a Path,
    pub compile_message_path: &'a Path,
    pub lang: Language,
}

pub struct CaseTask<'a> {
    pub working_dir: &'a Path,
    pub submission: &'a Submission,
    pub src_path: PathBuf,
    pub bin_path: Option<PathBuf>,
    pub case_index: u32,
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
    fn save_source(&self, source_code: &str, working_dir: &Path) -> WaResult<PathBuf>;
    fn compile(&self, task: CompileTask) -> WaResult<CompileResult>;
    fn run_case(&self, task: &CaseTask) -> WaResult<JudgeCaseResult>;
}

impl Language {
    pub fn get_broker(self) -> Box<dyn LanguageBroker> {
        unimplemented!()
    }

    fn get_limit(self) -> Option<Limit> {
        use crate::config::CompileLimit;
        use crate::GLOBAL_CONFIG;
        let limits: &HashMap<Language, CompileLimit> = &GLOBAL_CONFIG.compile_limit;
        limits.get(&self).map(|limit| Limit {
            time: limit.time.saturating_mul(1000),
            memory: limit.memory.saturating_mul(1024),
            output: limit.output.saturating_mul(1024),
            security_cfg_path: limit
                .security_cfg_path
                .as_ref()
                .map(|p: &PathBuf| p.as_path()),
        })
    }

    pub fn get_security_cfg(self) -> Option<&'static Path> {
        use crate::config::CompileLimit;
        use crate::GLOBAL_CONFIG;
        let limits: &HashMap<Language, CompileLimit> = &GLOBAL_CONFIG.compile_limit;
        limits.get(&self).and_then(|limit| {
            limit
                .security_cfg_path
                .as_ref()
                .map(|p: &PathBuf| p.as_path())
        })
    }
}
