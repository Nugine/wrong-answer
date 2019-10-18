mod bare_monitor;
pub use bare_monitor::BareMonitorSandBox;

use crate::lang::CaseTask;
use crate::types::*;
use crate::GLOBAL_CONFIG;

pub struct Target<'a> {
    pub working_dir: &'a Path,
    pub bin: &'a str,
    pub args: Vec<&'a str>,
    pub stdin: Option<&'a Path>,
    pub stdout: Option<&'a Path>,
    pub stderr: Option<&'a Path>,
}

pub struct Limit {
    pub time: MilliSecond,
    pub memory: KiloByte,
    pub output: KiloByte,
    pub security_cfg_path: Option<&'static Path>,
}

pub trait SandBox {
    fn run(&self, target: Target, limit: Option<&Limit>) -> WaResult<TargetStatus>;
}

impl Limit {
    pub fn no_effect() -> Option<&'static Self> {
        None
    }

    pub fn from_submission(submission: &Submission) -> Self {
        Limit {
            time: submission.time_limit.saturating_mul(1000),
            memory: submission
                .memory_limit
                .saturating_mul(2)
                .min(GLOBAL_CONFIG.memory_hard_limit)
                .saturating_mul(1024),
            output: GLOBAL_CONFIG.output_hard_limit.saturating_mul(1024),
            security_cfg_path: submission.lang.get_security_cfg(),
        }
    }
}

impl<'a> Target<'a> {
    pub fn direct<'b>(task: &'b CaseTask) -> Target<'b> {
        Target {
            working_dir: task.working_dir,
            bin: task.bin_path.as_ref().unwrap().to_str().unwrap(),
            args: vec![],
            stdin: Some(&task.stdin_path),
            stdout: Some(&task.userout_path),
            stderr: None,
        }
    }

    pub fn spj<'b>(task: &'b CaseTask) -> Target<'b> {
        Target {
            working_dir: task.working_dir,
            bin: task.spj_path.as_ref().unwrap().to_str().unwrap(),
            args: vec![
                task.stdout_path.to_str().unwrap(),
                task.userout_path.to_str().unwrap(),
            ],
            stdin: Some(&task.stdin_path),
            stdout: None,
            stderr: None,
        }
    }
}
