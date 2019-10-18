use crate::types::*;
use crate::GLOBAL_CONFIG;
use crossbeam_channel::{Receiver, Sender};
use std::path::{Path, PathBuf};

pub struct Worker<S: SandBox> {
    pub submission_receiver: Receiver<Submission>,
    pub update_sender: Sender<Update>,
    pub working_dir: PathBuf,
    pub sandbox: S,
}

macro_rules! handle {
    ($ret:expr,$fmt:expr) => {{
        match $ret {
            Err(e) => {
                log::error!($fmt, e);
                panic!(e)
            }
            Ok(r) => r,
        }
    }};
    (@custom $ret:expr,$fmt:expr,$($other:expr,)*) => {{
        match $ret {
            Err(e) => {
                log::error!($fmt, $($other)*);
                panic!(e)
            }
            Ok(r) => r,
        }
    }};
}

impl<S> Worker<S> 
where
    S:SandBox+Send+'static
{
    pub fn work(self) -> impl Fn() + Send + 'static {
        move || loop {
            let submission = handle!(
                self.submission_receiver.recv(),
                "submission sender is disconnected: {}"
            );

            let id = submission.id;

            let submission_dir = self.working_dir.join(id.to_string());
            handle!(@custom
                std::fs::create_dir_all(&submission_dir),
                "can not create dir: {:?}",
                &submission_dir,
            );

            if let Err(e) = self.handle_submission(submission, &submission_dir) {
                log::error!("system error: {}", e);
                self.send_update(Update::from_status(id, JudgeStatus::SE))
            }

            handle!(@custom
                std::fs::remove_dir_all(&submission_dir),
                "can not remove dir: {:?}",
                &submission_dir,
            );
        }
    }
}

impl<S:SandBox> Worker<S> {
    fn send_update(&self, update: Update) {
        handle!(
            self.update_sender.send(update),
            "updater is disconnected: {}"
        );
    }

    fn try_send_update(&self, update: Update) -> WaResult<()> {
        unimplemented!()
    }

    // TODO:
    fn handle_submission(&self, submission: Submission, dir: &Path) -> WaResult<()> {
        let lang = submission.language;

        // save source code
        let source_path = dir.join(lang.get_source_name());
        std::fs::write(&source_path, &submission.source_code)?;

        // Queuing -> Compiling
        self.try_send_update(Update::from_status(submission.id, JudgeStatus::Compiling))?;

        // compile
        let target_path = {
            if let Some(compiler) = lang.get_compiler() {
                let binary_path = dir.join(
                    submission
                        .language
                        .get_binary_name()
                        .expect("lang config error"),
                );
                let ce_path = dir.join(&GLOBAL_CONFIG.ce_filename);

                let task: CompileTask = CompileTask {
                    working_dir: dir,
                    source_path: source_path.to_str().unwrap(),
                    binary_path: binary_path.to_str().unwrap(),
                    ce_message_path: ce_path.to_str().unwrap(),
                };
                let limit = lang.get_limit();

                let status = compiler.compile(task, limit)?;
                let status = match status.code {
                    Some(0) => None,
                    Some(_) => Some(JudgeStatus::CE),
                    None => Some(JudgeStatus::CLE),
                };
                if let Some(status) = status {
                    let ce_message = std::fs::read_to_string(&ce_path)?;
                    let update = Update {
                        submission_id: submission.id,
                        status,
                        result: Some(JudgeResult::from_ce(ce_message)),
                    };
                    self.try_send_update(update)?;
                    return Ok(()); // final: CE | CLE
                }

                binary_path
            } else {
                source_path
            }
        };

        // Compiling -> Judging
        self.try_send_update(Update::from_status(submission.id, JudgeStatus::Judging))?;

        // run
        let mut case_results = <Vec<JudgeCaseResult>>::with_capacity(submission.problem.case_num as usize);
        let data_dir = Path::new(&GLOBAL_CONFIG.data_dir).join(submission.problem.id.to_string());
        let mut stdin  =data_dir.clone();
        let mut stdout  =data_dir.clone();
        for case in cases(submission.problem.case_num) {
            let (bin, args) = lang.get_target(&target_path);
            stdin.push(case.0);
            stdout.push(case.1);
            let target = Target {
                working_dir: dir,
                bin,
                args: &args,
                stdin: Some(stdin.to_str().unwrap()),
                stdout: Some(stdout.to_str().unwrap()),
                stderr: None,
            };
            let limit = Limit{
                time: submission.problem.time_limit,
                memory: submission.problem.memory_limit.saturating_mul(2).min(GLOBAL_CONFIG.memory_hard_limit),
                output: GLOBAL_CONFIG.output_limit,
                security_cfg_path: lang.get_security_cfg()
            };
            let target_status = self.sandbox.run(target,Some(limit))?;
            stdin.pop();
            stdout.pop();

            let cpu_time : MilliSecond = target_status.user_time+target_status.sys_time;
            let memory = target_status.memory;

            let mut case_result = JudgeCaseResult{
                time: cpu_time,
                memory,
                status: JudgeStatus::Judging,
            };

            if cpu_time > submission.problem.time_limit.saturating_mul(1000){
                case_result.status = JudgeStatus::TLE;
            }else if memory> submission.problem.memory_limit.saturating_mul(1024){
                case_result.status = JudgeStatus::MLE;
            }else if target_status.signal == Some(25){ // ENOTTY
                case_result.status = JudgeStatus::OLE;
            }else if is_runtime_error(&target_status){
                case_result.status = JudgeStatus::RE;
            }

            use crate::comparer::SimpleComparer;

            let comparer = SimpleComparer{ignore_trailing_space: true};


            // comparer.compare()
            unimplemented!();
            
        }

        unimplemented!()
    }
}

fn cases(case_num: u32) -> impl Iterator<Item = (String, String)> {
    (1..=case_num).map(|i| (format!("{}.in", i), format!("{}.out", i)))
}

fn is_runtime_error(status: &TargetStatus)->bool{
    match status.code{
        Some(0)=>false,
        _ => true,
    }
}