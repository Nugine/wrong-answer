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
            stderr: task.err_log_path.as_ref().map(|p|&**p)
        }
    }

    pub fn spj<'b>(_task: &'b CaseTask) -> Target<'b> {
        unimplemented!() // FIXME:
        // Target {
        //     working_dir: task.working_dir,
        //     bin: task.spj_path.as_ref().unwrap().to_str().unwrap().to_owned(),
        //     args: vec![
        //         to_string(&task.stdin_path),
        //         to_string(&task.stdout_path),
        //         to_string(&task.userout_path),
        //     ],
        //     stdin: None,
        //     stdout: None,
        //     stderr: None,
        // }
    }

    pub fn wrap_interact(self, act_path:&'a Path,aupipe: &'a str, uapipe: &'a str, err_log_path: Option<&'a Path>)->Self{
        let bin = "wa-interact".into();
        let mut args = vec![
            "--actin".into(),
            self.stdin.map(to_string).unwrap(),
            "--actout".into(),
            self.stdout.map(to_string).unwrap(),
            "--actpath".into(),
            to_string(act_path),
            "--aupipe".into(),
            aupipe.into(),
            "--uapipe".into(),
            uapipe.into(),
            "--".into(),
            self.bin,
        ];
        args.extend(self.args);

        Target{
            working_dir: self.working_dir,
            bin,
            args,
            stdin: None,
            stdout: None,
            stderr: err_log_path,
        }
    }
}

fn to_string(path: &Path)->String{
    path.to_str().unwrap().to_owned()
}
