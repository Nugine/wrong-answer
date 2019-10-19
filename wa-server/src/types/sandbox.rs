use super::*;
use crate::GLOBAL_CONFIG;

pub struct Target<'a> {
    pub working_dir: &'a Path,
    pub bin: String,
    pub args: Vec<String>,
    pub stdin: Option<&'a Path>,
    pub stdout: Option<&'a Path>,
    pub stderr: Option<&'a Path>,
}

#[derive(Deserialize)]
pub struct Limit {
    pub time: MilliSecond,
    pub memory: KiloByte,
    pub output: KiloByte,
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
        }
    }
}

impl<'a> Target<'a> {
    pub fn direct<'b>(task: &'b CaseTask) -> Target<'b> {
        Target {
            working_dir: task.working_dir,
            bin: task
                .working_dir
                .join(task.bin_filename.unwrap())
                .to_str()
                .unwrap()
                .to_owned(),
            args: vec![],
            stdin: Some(&task.stdin_path),
            stdout: Some(&task.userout_path),
            stderr: None,
        }
    }

    pub fn spj<'b>(task: &'b CaseTask) -> Target<'b> {
        Target {
            working_dir: task.working_dir,
            bin: task.spj_path.as_ref().unwrap().to_str().unwrap().to_owned(),
            args: vec![
                task.stdin_path.to_str().unwrap().to_owned(),
                task.stdout_path.to_str().unwrap().to_owned(),
                task.userout_path.to_str().unwrap().to_owned(),
            ],
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }
}
