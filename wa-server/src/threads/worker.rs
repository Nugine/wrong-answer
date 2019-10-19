use super::*;
use crate::types::*;
use crate::utils::compare;
use crate::GLOBAL_CONFIG;

#[derive(Clone)]
pub struct Worker<S: SandBox> {
    pub submission_receiver: Receiver<Submission>,
    pub update_sender: Sender<Update>,
    pub workspace: PathBuf,
    pub sandbox: S,
    pub data_lock: Arc<DataLock>,
}

impl<S> Worker<S>
where
    S: SandBox + Send + 'static,
{
    pub fn work(self) -> impl Fn() + Send + 'static {
        move || loop {
            let submission = handle!(
                self.submission_receiver.recv(),
                "submission sender is disconnected: {}"
            );

            let submission_id = submission.id;
            let working_dir = self.workspace.join(submission_id.to_string());

            handle!(@custom
                std::fs::create_dir_all(&working_dir),
                "can not create dir: {:?}",
                &working_dir,
            );

            if let Err(e) = self.handle_submission(submission, &working_dir) {
                log::error!("system error: {}", e);
                self.send_update(Update {
                    submission_id,
                    status: JudgeStatus::SE,
                    result: Some(JudgeResult::zero()),
                });
                // NOTE: SE
            }

            // don't remove working_dir in debug mode
            if !cfg!(debug_assertions) {
                handle!(@custom
                    std::fs::remove_dir_all(&working_dir),
                    "can not remove dir: {:?}",
                    &working_dir,
                );
            }
        }
    }

    fn handle_submission(&self, submission: Submission, working_dir: &Path) -> WaResult<()> {
        let broker = submission.lang.get_broker();

        // save source code
        let (src_filename, bin_filename) = broker.filename();
        let src_path = working_dir.join(src_filename);
        std::fs::write(src_path, &submission.source_code)?;

        // Queuing -> Compiling
        self.try_send_update(submission.update(JudgeStatus::Compiling))?;

        // compile
        let ce_filename = "ce.txt";
        let target = broker.compile(working_dir, ce_filename);
        let limit = GLOBAL_CONFIG.compile_limit.as_ref();
        let status = self.sandbox.run(target, limit)?;
        match status.code {
            Some(0) => {}
            Some(_) => {
                let msg = std::fs::read_to_string(working_dir.join(ce_filename))?;
                let result = JudgeResult::from_ce(msg);
                let update = submission.final_update(JudgeStatus::CE, result);
                return self.try_send_update(update); // NOTE: CE
            }
            None => {
                let result = JudgeResult::zero();
                let update = submission.final_update(JudgeStatus::CLE, result);
                return self.try_send_update(update); // NOTE: CLE
            }
        };

        // Compiling -> Judging
        self.try_send_update(submission.update(JudgeStatus::Judging))?;

        let data_guard = self.data_lock.read().expect("data lock error");

        let data_dir: &Path = &GLOBAL_CONFIG.data_dir;
        let mut case_task = CaseTask {
            working_dir,
            submission: &submission,
            src_filename,
            bin_filename,
            case_index: 0,
            stdin_path: data_dir.join(submission.problem_id.to_string()),
            stdout_path: data_dir.join(submission.problem_id.to_string()),
            userout_path: working_dir.join("userout.out"),
            act_path: if submission.judge_type == JudgeType::Interactive {
                Some(working_dir.join("act"))
            } else {
                None
            },
            spj_path: if submission.judge_type == JudgeType::SpecialJudge {
                Some(working_dir.join("spj"))
            } else {
                None
            },
        };

        let mut status = JudgeStatus::AC;
        let mut result = JudgeResult::zero();
        let cases =
            (1..=submission.case_num).map(|i| (i, format!("{}.in", i), format!("{}.out", i)));

        for (i, input, output) in cases {
            case_task.case_index = i;
            case_task.stdin_path.push(input);
            case_task.stdout_path.push(output);

            let res = self.run_case(broker.as_ref(), &case_task).map_err(|err| {
                log::error!(
                    "system error: {}, submission_id = {}, problem_id = {}, case = {}",
                    err,
                    submission.id,
                    submission.problem_id,
                    case_task.case_index,
                );
                err
            })?;

            if res.status != JudgeStatus::AC {
                status = res.status;
            }

            result.time += res.time;
            result.memory += res.memory;
            result.cases.push(res);

            case_task.stdin_path.pop();
            case_task.stdout_path.pop();
        }
        drop(data_guard);

        self.try_send_update(submission.final_update(status, result))
    }

    fn run_case(&self, broker: &dyn LanguageBroker, task: &CaseTask) -> WaResult<JudgeCaseResult> {
        let target = broker.run_case(task);
        let limit = Limit::from_submission(&task.submission);
        let target_status = self.sandbox.run(target, Some(&limit))?;

        let (time, memory, status) = parse_status(&target_status, &limit);

        if let Some(status) = status {
            return Ok(JudgeCaseResult {
                time,
                memory,
                status,
            });
        }

        let status = if task.spj_path.is_some() {
            let target = Target::spj(task);
            let target_status = self.sandbox.run(target, Some(&limit))?;
            match target_status.code {
                Some(0) => JudgeStatus::AC,
                Some(1) => JudgeStatus::WA,
                Some(2) => JudgeStatus::PE,
                _ => {
                    log::error!("special judge error: code = {:?}, signal = {:?}, submission_id = {}, case = {}",
                        target_status.code,
                        target_status.signal,
                        task.submission.id,
                        task.case_index,
                    );
                    JudgeStatus::WA
                }
            }
        } else {
            let ignore_trailing_space = match task.submission.judge_type {
                JudgeType::Strict => false,
                JudgeType::IgnoreTrialingSpace => true,
                _ => unreachable!(),
            };
            let cmp = compare(ignore_trailing_space, &task.stdout_path, &task.userout_path)?;
            cmp.to_status()
        };

        Ok(JudgeCaseResult {
            time,
            memory,
            status,
        })
    }

    fn send_update(&self, update: Update) {
        handle!(
            self.update_sender.send(update),
            "updater is disconnected: {}"
        );
    }

    fn try_send_update(&self, update: Update) -> WaResult<()> {
        self.update_sender
            .send(update)
            .map_err(|_| WaError::Channel("updater is disconnected"))
    }
}

pub fn parse_status(
    status: &TargetStatus,
    limit: &Limit,
) -> (MilliSecond, KiloByte, Option<JudgeStatus>) {
    let cpu_time = status.user_time + status.sys_time;
    let memory = status.memory;

    let gen = |status| (cpu_time, memory, Some(status));

    if cpu_time > limit.time {
        return gen(JudgeStatus::TLE);
    }

    if memory > limit.memory {
        return gen(JudgeStatus::MLE);
    }

    // NOTE: ENOTTY 25
    if status.signal == Some(25) {
        return gen(JudgeStatus::OLE);
    }

    if status.code != Some(0) {
        return gen(JudgeStatus::RE);
    }

    (cpu_time, memory, None)
}
